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

#include "GameplayEffectExtension.h"
#include "GameplayEffectTypes.h"
#include "GameplayEffect.h"
#include "Net/UnrealNetwork.h"

void AGardumPlayerState::GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const
{
	Super::GetLifetimeReplicatedProps(OutLifetimeProps);

	DOREPLIFETIME(AGardumPlayerState, Kills);
	DOREPLIFETIME(AGardumPlayerState, Deaths);
	DOREPLIFETIME(AGardumPlayerState, Damage);
	DOREPLIFETIME(AGardumPlayerState, Healing);
}

void AGardumPlayerState::CopyProperties(APlayerState* PlayerState)
{
	Super::CopyProperties(PlayerState);

	if (PlayerState == nullptr)
	{
		return;
	}

	auto* GardumPlayerState = CastChecked<AGardumPlayerState>(PlayerState);
	GardumPlayerState->Kills = Kills;
	GardumPlayerState->Deaths = Deaths;
	GardumPlayerState->Damage = Damage;
	GardumPlayerState->Healing = Healing;
}

void AGardumPlayerState::OverrideWith(APlayerState* PlayerState)
{
	Super::OverrideWith(PlayerState);

	if (PlayerState == nullptr)
	{
		return;
	}

	auto* GardumPlayerState = CastChecked<AGardumPlayerState>(PlayerState);
	Kills = GardumPlayerState->Kills;
	Deaths = GardumPlayerState->Deaths;
	Damage = GardumPlayerState->Damage;
	Healing = GardumPlayerState->Healing;
}

void AGardumPlayerState::OnHealthChanged(const FOnAttributeChangeData& Data)
{
	if (Data.NewValue <= 0)
	{
		AddDeath();
	}

	const auto* Instigator = Cast<APawn>(Data.GEModData->EffectSpec.GetContext().GetInstigator());
	if (Instigator == nullptr)
	{
		return;
	}

	auto* InstigatorState = Instigator->GetPlayerStateChecked<AGardumPlayerState>();
	if (Data.NewValue <= 0)
	{
		InstigatorState->AddKill();
	}
	const float Difference = Data.NewValue - Data.OldValue;
	if (Difference < 0)
	{
		InstigatorState->AddDamage(static_cast<uint32>(-Difference));
	}
	else
	{
		InstigatorState->AddHealing(static_cast<uint32>(Difference));
	}
}

TMulticastDelegate<void(int16)>& AGardumPlayerState::OnKill()
{
	return KillsChangedDelegate;
}

TMulticastDelegate<void(uint16)>& AGardumPlayerState::OnDeath()
{
	return DeathsChangedDelegate;
}

TMulticastDelegate<void(uint32)>& AGardumPlayerState::OnDamage()
{
	return DamageChangedDelegate;
}

TMulticastDelegate<void(uint32)>& AGardumPlayerState::OnHealing()
{
	return HealingChangedDelegate;
}

int16 AGardumPlayerState::GetKills() const
{
	return Kills;
}

uint16 AGardumPlayerState::GetDeaths() const
{
	return Deaths;
}

uint32 AGardumPlayerState::GetDamage() const
{
	return Damage;
}

uint32 AGardumPlayerState::GetHealing() const
{
	return Healing;
}

void AGardumPlayerState::AddKill()
{
	++Kills;
	KillsChangedDelegate.Broadcast(Kills);
}

void AGardumPlayerState::AddDeath()
{
	++Deaths;
	DeathsChangedDelegate.Broadcast(Deaths);
}

void AGardumPlayerState::AddDamage(uint32 Value)
{
	Damage += Value;
	DamageChangedDelegate.Broadcast(Damage);
}

void AGardumPlayerState::AddHealing(uint32 Value)
{
	Healing += Value;
	HealingChangedDelegate.Broadcast(Healing);
}

void AGardumPlayerState::OnRep_Kills()
{
	KillsChangedDelegate.Broadcast(Kills);
}

void AGardumPlayerState::OnRep_Deaths()
{
	DeathsChangedDelegate.Broadcast(Deaths);
}

void AGardumPlayerState::OnRep_Damage()
{
	DamageChangedDelegate.Broadcast(Damage);
}

void AGardumPlayerState::OnRep_Health()
{
	HealingChangedDelegate.Broadcast(Healing);
}
