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

#include "Blueprint/UserWidget.h"
#include "CoreMinimal.h"
#include "Heroes/HeroTypes.h"

#include "HeroHUD.generated.h"

class UProgressBar;
class UAbilityIcon;
struct FGameplayAbilityActorInfo;
struct FGameplayAbilitySpec;
struct FOnAttributeChangeData;

UCLASS()
class GARDUM_API UHeroHUD : public UUserWidget
{
	GENERATED_BODY() // NOLINT

public:
	void SetHealth(float CurrentValue, float BaseValue);
	void OnHealthAttributeChanged(const FOnAttributeChangeData& Data);
	void SetActorInfo(const TSharedPtr<const FGameplayAbilityActorInfo>& ActorInfo);
	void SetAbility(const FGameplayAbilitySpec* AbilitySpec, AbilityAction Action);

private:
	UPROPERTY(meta = (BindWidget))
	UProgressBar* HealthBar = nullptr;

	UPROPERTY(meta = (BindWidget))
	UAbilityIcon* MainAttackIcon = nullptr;

	UPROPERTY(meta = (BindWidget))
	UAbilityIcon* Ability1Icon = nullptr;

	UPROPERTY(meta = (BindWidget))
	UAbilityIcon* Ability2Icon = nullptr;

	UPROPERTY(meta = (BindWidget))
	UAbilityIcon* Ability3Icon = nullptr;

	UPROPERTY(meta = (BindWidget))
	UAbilityIcon* UltimateIcon = nullptr;
};
