/*
 *  Copyright © 2021 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a get of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

#include "Heroes/Hero.h"

#include "Abilities/GameplayAbilityTypes.h"
#include "AbilitySystemComponent.h"
#include "Camera/CameraComponent.h"
#include "Components/CapsuleComponent.h"
#include "Components/InputComponent.h"
#include "Core/GardumAttributeSet.h"
#include "Core/GardumPlayerState.h"
#include "Core/Tags.h"
#include "GameFramework/CharacterMovementComponent.h"
#include "GameFramework/Controller.h"
#include "GameFramework/SpringArmComponent.h"
#include "GameplayEffectExtension.h"
#include "Heroes/HeroGameplayAbility.h"

AHero::AHero()
	: SpringArm(CreateDefaultSubobject<USpringArmComponent>(TEXT("SpringArm")))
	, FollowCamera(CreateDefaultSubobject<UCameraComponent>(TEXT("FollowCamera")))
{
	GetMesh()->SetCollisionProfileName("Ragdoll");

	// Set size for collision capsule
	GetCapsuleComponent()->InitCapsuleSize(42.0f, 96.0f);

	// Don't rotate when the controller rotates. Let that just affect the camera.
	bUseControllerRotationPitch = false;
	bUseControllerRotationYaw = false;
	bUseControllerRotationRoll = false;

	// Configure character movement
	GetCharacterMovement()->bOrientRotationToMovement = true; // Character moves in the direction of input...
	GetCharacterMovement()->RotationRate = FRotator(0.0f, 540.0f, 0.0f); // ...at this rotation rate
	GetCharacterMovement()->JumpZVelocity = 600.0f;
	GetCharacterMovement()->AirControl = 0.2f;

	// Setup spring arm (pulls in towards the player if there is a collision)
	SpringArm->SetupAttachment(RootComponent);
	SpringArm->TargetArmLength = 300.0f; // The camera follows at this distance behind the character
	SpringArm->bUsePawnControlRotation = true; // Rotate the arm based on the controller

	// Attach the camera to the end of the boom and let the boom adjust to match the controller orientation
	FollowCamera->SetupAttachment(SpringArm, USpringArmComponent::SocketName);
	FollowCamera->bUsePawnControlRotation = false; // Camera does not rotate relative to arm

	AbilitySystem = CreateDefaultSubobject<UAbilitySystemComponent>(TEXT("AbilitySystem"));
	AbilitySystem->SetIsReplicated(true);
	AbilitySystem->SetReplicationMode(EGameplayEffectReplicationMode::Minimal);
}

UAbilitySystemComponent* AHero::GetAbilitySystemComponent() const
{
	return AbilitySystem;
}

void AHero::PossessedBy(AController* NewController)
{
	Super::PossessedBy(NewController);

	SetupAbilitySystem();
	for (const auto& [Action, Ability] : DefaultAbilities)
	{
		if (ensureAlwaysMsgf(Ability != nullptr, TEXT("Ability was not specified")))
		{
			AbilitySystem->GiveAbility(FGameplayAbilitySpec(Ability, 1, static_cast<int32>(Action)));
		}
	}
}

void AHero::OnRep_PlayerState()
{
	Super::OnRep_PlayerState();
	SetupAbilitySystem();
}

void AHero::SetupPlayerInputComponent(UInputComponent* PlayerInputComponent)
{
	// Set up gameplay key bindings
	PlayerInputComponent->BindAction("Jump", IE_Pressed, this, &ACharacter::Jump);
	PlayerInputComponent->BindAction("Jump", IE_Released, this, &ACharacter::StopJumping);

	PlayerInputComponent->BindAxis("MoveForward", this, &AHero::MoveForward);
	PlayerInputComponent->BindAxis("MoveRight", this, &AHero::MoveRight);

	// We have 2 versions of the rotation bindings to handle different kinds of devices differently
	// "turn" handles devices that provide an absolute delta, such as a mouse.
	// "turnrate" is for devices that we choose to treat as a rate of change, such as an analog joystick
	PlayerInputComponent->BindAxis("Turn", this, &APawn::AddControllerYawInput);
	PlayerInputComponent->BindAxis("TurnRate", this, &AHero::TurnAtRate);
	PlayerInputComponent->BindAxis("LookUp", this, &APawn::AddControllerPitchInput);
	PlayerInputComponent->BindAxis("LookUpRate", this, &AHero::LookUpAtRate);

	// Bind ability keys
	AbilitySystem->BindAbilityActivationToInputComponent(PlayerInputComponent, FGameplayAbilityInputBinds(FString("ConfirmTarget"), FString("CancelTarget"), FString("AbilityAction")));
}

void AHero::TurnAtRate(float Rate)
{
	// Calculate delta for this frame from the rate information
	AddControllerYawInput(Rate * BaseTurnRate * GetWorld()->GetDeltaSeconds());
}

void AHero::LookUpAtRate(float Rate)
{
	// Calculate delta for this frame from the rate information
	AddControllerPitchInput(Rate * BaseLookUpRate * GetWorld()->GetDeltaSeconds());
}

void AHero::MoveForward(float Value)
{
	if (Controller != nullptr && Value != 0.0f)
	{
		// Find out which way is forward
		const FRotator Rotation = Controller->GetControlRotation();
		const FRotator YawRotation(0, Rotation.Yaw, 0);

		// Get forward direction
		const FVector Direction = FRotationMatrix(YawRotation).GetUnitAxis(EAxis::X);
		AddMovementInput(Direction, Value);
	}
}

void AHero::MoveRight(float Value)
{
	if (Controller != nullptr && Value != 0.0f)
	{
		// Find out which way is right
		const FRotator Rotation = Controller->GetControlRotation();
		const FRotator YawRotation(0, Rotation.Yaw, 0);

		// Get right direction
		const FVector Direction = FRotationMatrix(YawRotation).GetUnitAxis(EAxis::Y);
		AddMovementInput(Direction, Value);
	}
}

void AHero::OnHealthChanged(const FOnAttributeChangeData& Data)
{
	if (Data.NewValue <= 0)
	{
		AbilitySystem->AddLooseGameplayTag(Tags::Dead);
	}

	if (GetNetMode() > NM_ListenServer)
	{
		// Data.GEMode is available only on the server, so we update the PlayerState statistics only on it and replicate the values to clients
		return;
	}

	const auto* Instigator = Cast<APawn>(Data.GEModData->EffectSpec.GetContext().GetInstigator());
	if (Instigator == nullptr)
	{
		return;
	}

	if (auto* State = Cast<AGardumPlayerState>(Instigator->GetPlayerState()); State)
	{
		const auto Difference = static_cast<uint32>(Data.NewValue - Data.OldValue);
		if (Difference < 0)
		{
			State->AddDamage(-Difference);
		}
		else
		{
			State->AddHealing(Difference);
		}
	}
}

void AHero::SetupAbilitySystem()
{
	AbilitySystem->InitAbilityActorInfo(this, this);
	AbilitySystem->GetGameplayAttributeValueChangeDelegate(UGardumAttributeSet::GetHealthAttribute()).AddUObject(this, &AHero::OnHealthChanged);
	AbilitySystem->RegisterGameplayTagEvent(Tags::Dead, EGameplayTagEventType::NewOrRemoved).AddUObject(this, &AHero::OnDeadTagChanged);
}

void AHero::OnDeadTagChanged([[maybe_unused]] FGameplayTag Tag, int32 NewCount)
{
	if (NewCount == 1)
	{
		GetCapsuleComponent()->SetCollisionEnabled(ECollisionEnabled::NoCollision);
		GetMesh()->SetSimulatePhysics(true);
		if (auto *Controller = Cast<APlayerController>(GetController()); Controller != nullptr)
		{
			DisableInput(Controller);
		}
	}
}
