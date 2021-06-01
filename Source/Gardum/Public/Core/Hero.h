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

#pragma once

#include "AbilitySystemInterface.h"
#include "CoreMinimal.h"
#include "GameFramework/Character.h"

#include "Hero.generated.h"

class USpringArmComponent;
class UCameraComponent;
class UAbilitySystemComponent;
class UGameplayAbility;

UENUM()
enum class AbilityActions : int32
{
	MainAttack,
	Ability1,
	Ability2,
	Ability3,
	Ultimate,
};

UCLASS(config = Game)
class GARDUM_API AHero : public ACharacter, public IAbilitySystemInterface
{
	GENERATED_BODY() // NOLINT

public:
	AHero();

	UAbilitySystemComponent* GetAbilitySystemComponent() const override;

	void PostInitializeComponents() override;
	void OnRep_PlayerState() override;

protected:
	void SetupPlayerInputComponent(UInputComponent* PlayerInputComponent) override;

private:
	/**
	 * Called via input to turn at a given rate.
	 * @param Rate	This is a normalized rate, i.e. 1.0 means 100% of desired turn rate
	 */
	void TurnAtRate(float Rate);

	/**
	 * Called via input to turn look up/down at a given rate.
	 * @param Rate	This is a normalized rate, i.e. 1.0 means 100% of desired turn rate
	 */
	void LookUpAtRate(float Rate);

	/** Called for forwards/backward input */
	void MoveForward(float Value);

	/** Called for side to side input */
	void MoveRight(float Value);

	/** Camera boom positioning the camera behind the character */
	UPROPERTY(VisibleAnywhere, Category = "Camera")
	USpringArmComponent* SpringArm;

	/** Follow camera */
	UPROPERTY(VisibleAnywhere, Category = "Camera")
	UCameraComponent* FollowCamera;

	UPROPERTY(VisibleAnywhere, Category = "Abilities")
	UAbilitySystemComponent* AbilitySystem;

	UPROPERTY(EditAnywhere, Category = "Abilities")
	TMap<AbilityActions, TSubclassOf<UGameplayAbility>> DefaultAbilities;

	/** Base turn rate, in deg/sec. Other scaling may affect final turn rate. */
	static constexpr float BaseTurnRate = 45.0f;

	/** Base look up/down rate, in deg/sec. Other scaling may affect final rate. */
	static constexpr float BaseLookUpRate = 45.0f;
};
