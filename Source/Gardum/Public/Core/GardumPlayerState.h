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

class UAbilitySystemComponent;
struct FOnAttributeChangeData;

UCLASS()
class GARDUM_API AGardumPlayerState : public APlayerState
{
	GENERATED_BODY() // NOLINT

public:
	void PostInitializeComponents() override;
	void GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const override;

	void CopyProperties(class APlayerState* PlayerState) override;
	void OverrideWith(class APlayerState* PlayerState) override;

	float GetDamage() const;
	float GetHealing() const;

private:
	UFUNCTION()
	void SetAbilitySystem(UAbilitySystemComponent* NewAbilitySystem);

	static void UpdateHealthStatistic(const FOnAttributeChangeData& Data);

	UPROPERTY()
	UAbilitySystemComponent* AbilitySystem = nullptr;

	UPROPERTY(Replicated)
	float Damage = 0;

	UPROPERTY(Replicated)
	float Healing = 0;
};
