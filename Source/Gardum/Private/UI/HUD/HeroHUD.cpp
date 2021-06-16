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

#include "UI/HUD/HeroHUD.h"

#include "Components/ProgressBar.h"
#include "GameplayEffectTypes.h"
#include "UI/HUD/AbilityIcon.h"

void UHeroHUD::SetHealth(float CurrentValue, float BaseValue)
{
	HealthBar->SetPercent(CurrentValue / BaseValue);
}

void UHeroHUD::OnHealthAttributeChanged(const FOnAttributeChangeData& Data)
{
	HealthBar->SetPercent(HealthBar->Percent * Data.NewValue / Data.OldValue);
}

void UHeroHUD::SetActorInfo(const TSharedPtr<const FGameplayAbilityActorInfo> &ActorInfo)
{
	MainAttackIcon->SetActorInfo(ActorInfo);
	Ability1Icon->SetActorInfo(ActorInfo);
	Ability2Icon->SetActorInfo(ActorInfo);
	Ability3Icon->SetActorInfo(ActorInfo);
	UltimateIcon->SetActorInfo(ActorInfo);
}

void UHeroHUD::SetAbility(const FGameplayAbilitySpec* AbilitySpec, AbilityAction Action)
{
	switch (Action)
	{
		case AbilityAction::MainAttack:
			MainAttackIcon->SetAbility(AbilitySpec);
			break;
		case AbilityAction::Ability1:
			Ability1Icon->SetAbility(AbilitySpec);
			break;
		case AbilityAction::Ability2:
			Ability2Icon->SetAbility(AbilitySpec);
			break;
		case AbilityAction::Ability3:
			Ability3Icon->SetAbility(AbilitySpec);
			break;
		case AbilityAction::Ultimate:
			UltimateIcon->SetAbility(AbilitySpec);
			break;
	}
}
