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

#include "CoreMinimal.h"
#include "GameFramework/PlayerState.h"

#include "GardumPlayerState.generated.h"

struct FOnAttributeChangeData;

UCLASS()
class GARDUM_API AGardumPlayerState : public APlayerState
{
	GENERATED_BODY() // NOLINT

public:
	void GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const override;

	void CopyProperties(APlayerState* PlayerState) override;
	void OverrideWith(APlayerState* PlayerState) override;

	void OnHealthChanged(const FOnAttributeChangeData& Data);

	TMulticastDelegate<void(int16)>& OnKill();
	TMulticastDelegate<void(uint16)>& OnDeath();
	TMulticastDelegate<void(uint32)>& OnDamage();
	TMulticastDelegate<void(uint32)>& OnHealing();

	int16 GetKills() const;
	uint16 GetDeaths() const;
	uint32 GetDamage() const;
	uint32 GetHealing() const;

private:
	void AddKill();
	void AddDeath();
	void AddDamage(uint32 Value);
	void AddHealing(uint32 Value);

	UFUNCTION()
	void OnRep_Kills();

	UFUNCTION()
	void OnRep_Deaths();

	UFUNCTION()
	void OnRep_Damage();

	UFUNCTION()
	void OnRep_Health();

	UPROPERTY(ReplicatedUsing = OnRep_Kills)
	int16 Kills = 0;

	UPROPERTY(ReplicatedUsing = OnRep_Deaths)
	uint16 Deaths = 0;

	UPROPERTY(ReplicatedUsing = OnRep_Damage)
	uint32 Damage = 0;

	UPROPERTY(ReplicatedUsing = OnRep_Health)
	uint32 Healing = 0;

	TMulticastDelegate<void(int16)> KillsChangedDelegate;
	TMulticastDelegate<void(uint16)> DeathsChangedDelegate;
	TMulticastDelegate<void(uint32)> DamageChangedDelegate;
	TMulticastDelegate<void(uint32)> HealingChangedDelegate;
};
