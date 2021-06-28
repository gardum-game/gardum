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

#include "Net/UnrealNetwork.h"

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

TMulticastDelegate<void(float)>& AGardumPlayerState::OnDamage()
{
	return DamageChangedDelegate;
}

TMulticastDelegate<void(float)>& AGardumPlayerState::OnHealing()
{
	return HealingChangedDelegate;
}

void AGardumPlayerState::AddDamage(float Value)
{
	Damage += Value;
	DamageChangedDelegate.Broadcast(Damage);
}

void AGardumPlayerState::AddHealing(float Value)
{
	Healing += Value;
	HealingChangedDelegate.Broadcast(Healing);
}

void AGardumPlayerState::OnRep_Damage()
{
	DamageChangedDelegate.Broadcast(Damage);
}

void AGardumPlayerState::OnRep_Health()
{
	HealingChangedDelegate.Broadcast(Healing);
}
