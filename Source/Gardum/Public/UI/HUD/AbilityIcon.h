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

#include "AbilityIcon.generated.h"

class UImage;
class UProgressBar;
class UNumberTextBlock;
struct FGameplayAbilitySpec;
struct FGameplayAbilityActorInfo;

UCLASS()
class GARDUM_API UAbilityIcon : public UUserWidget
{
	GENERATED_BODY() // NOLINT

public:
	void NativeTick(const FGeometry& MyGeometry, float InDeltaTime) override;

	void SetAbility(const FGameplayAbilitySpec* NewAbilitySpec, const TSharedPtr<const FGameplayAbilityActorInfo> &NewActorInfo);

private:
	UPROPERTY(meta = (BindWidget))
	UImage* Icon;

	UPROPERTY(meta = (BindWidget))
	UProgressBar* Cooldown;

	UPROPERTY(meta = (BindWidget))
	UNumberTextBlock* CooldownText;

	const FGameplayAbilitySpec* AbilitySpec = nullptr;
	TSharedPtr<const FGameplayAbilityActorInfo> ActorInfo;
};
