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

#include "Core/GardumPlayerState.h"

#include "AbilitySystemComponent.h"
#include "Core/GardumAttributeSet.h"
#include "Core/GardumPlayerController.h"
#include "GameplayEffectExtension.h"
#include "Net/UnrealNetwork.h"

void AGardumPlayerState::PostInitializeComponents()
{
	Super::PostInitializeComponents();

	if (HasAuthority())
	{
		Cast<AGardumPlayerController>(GetOwner())->OnAbilitySystemChanged().AddUObject(this, &AGardumPlayerState::SetAbilitySystem);
	}
}

void AGardumPlayerState::GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const
{
	Super::GetLifetimeReplicatedProps(OutLifetimeProps);

	DOREPLIFETIME(AGardumPlayerState, Damage);
	DOREPLIFETIME(AGardumPlayerState, Healing);
}

void AGardumPlayerState::CopyProperties(class APlayerState* PlayerState)
{
	Super::CopyProperties(PlayerState);

	if (PlayerState == nullptr)
	{
		return;
	}

	if (auto* GardumPlayerState = Cast<AGardumPlayerState>(PlayerState); GardumPlayerState != nullptr)
	{
		GardumPlayerState->Damage = Damage;
		GardumPlayerState->Healing = Healing;
	}
}

void AGardumPlayerState::OverrideWith(class APlayerState* PlayerState)
{
	Super::OverrideWith(PlayerState);

	if (PlayerState == nullptr)
	{
		return;
	}

	if (auto* GardumPlayerState = Cast<AGardumPlayerState>(PlayerState); GardumPlayerState != nullptr)
	{
		Damage = GardumPlayerState->Damage;
		Healing = GardumPlayerState->Healing;
	}
}

float AGardumPlayerState::GetDamage() const
{
	return Damage;
}

float AGardumPlayerState::GetHealing() const
{
	return Damage;
}

void AGardumPlayerState::SetAbilitySystem(UAbilitySystemComponent* NewAbilitySystem)
{
	const FGameplayAttribute HealthAttribute = UGardumAttributeSet::GetHealthAttribute();

	// Cleanup previous connections
	if (AbilitySystem != nullptr)
	{
		AbilitySystem->GetGameplayAttributeValueChangeDelegate(HealthAttribute).RemoveAll(this);
		AbilitySystem = nullptr;
	}

	AbilitySystem = NewAbilitySystem;
	if (AbilitySystem != nullptr)
	{
		AbilitySystem->GetGameplayAttributeValueChangeDelegate(HealthAttribute).AddStatic(&AGardumPlayerState::UpdateHealthStatistic);
	}
}

void AGardumPlayerState::UpdateHealthStatistic(const FOnAttributeChangeData& Data)
{
	const auto* Instigator = Cast<APawn>(Data.GEModData->EffectSpec.GetContext().GetInstigator());
	if (Instigator == nullptr)
	{
		return;
	}

	if (auto* State = Cast<AGardumPlayerState>(Instigator->GetPlayerState()); State)
	{
		const float Difference = Data.NewValue - Data.OldValue;
		if (Difference < 0)
		{
			State->Damage -= Difference;
		}
		else
		{
			State->Healing += Difference;
		}
	}
}
