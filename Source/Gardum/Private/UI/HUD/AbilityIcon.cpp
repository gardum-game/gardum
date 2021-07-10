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

#include "UI/HUD/AbilityIcon.h"

#include "Components/Image.h"
#include "Components/ProgressBar.h"
#include "UI/NumberTextBlock.h"
#include "Heroes/HeroGameplayAbility.h"

void UAbilityIcon::NativeTick(const FGeometry& MyGeometry, float InDeltaTime)
{
	Super::NativeTick(MyGeometry, InDeltaTime);

	if (AbilitySpec == nullptr)
	{
		return;
	}

	float TimeRemaining = 0;
	float CooldownDuration = 0;
	AbilitySpec->Ability->GetCooldownTimeRemainingAndDuration(AbilitySpec->Handle, ActorInfo.Get(), TimeRemaining, CooldownDuration);

	Cooldown->SetPercent(CooldownDuration == 0 ? 0 : TimeRemaining / CooldownDuration);
	if (const auto RoundedTime = static_cast<int>(FMath::RoundFromZero(TimeRemaining)); RoundedTime != 0)
	{
		CooldownText->SetNumber(RoundedTime);
	}
	else
	{
		CooldownText->SetText({});
	}
}

void UAbilityIcon::SetAbility(const FGameplayAbilitySpec* NewAbilitySpec, const TSharedPtr<const FGameplayAbilityActorInfo> &NewActorInfo)
{
	ActorInfo = NewActorInfo;
	AbilitySpec = NewAbilitySpec;
	if (AbilitySpec == nullptr)
	{
		return;
	}

	if (UTexture2D* AbilityIcon = CastChecked<UHeroGameplayAbility>(AbilitySpec->Ability)->GetIcon(); AbilityIcon != nullptr)
	{
		Icon->SetBrushFromTexture(AbilityIcon);
	}
}
