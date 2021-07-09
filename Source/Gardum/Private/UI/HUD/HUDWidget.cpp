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

#include "UI/HUD/HUDWidget.h"

#include "AbilitySystemComponent.h"
#include "Core/GardumAttributeSet.h"
#include "UI/HUD/AbilityIcon.h"
#include "UI/HUD/AttributeBar.h"

void UHUDWidget::SetAbilitySystem(UAbilitySystemComponent* AbilitySystem)
{
	HealthBar->SetAttribute(AbilitySystem, UGardumAttributeSet::GetHealthAttribute());

	const TArray<FGameplayAbilitySpec>& Abilities = AbilitySystem->GetActivatableAbilities();
	for (int i = 0; i < Abilities.Num(); ++i)
	{
		GetAbility(static_cast<AbilityAction>(i))->SetAbility(&Abilities[i], AbilitySystem->AbilityActorInfo);
	}
}

UAbilityIcon* UHUDWidget::GetAbility(AbilityAction Action)
{
	switch (Action)
	{
		case AbilityAction::MainAttack:
			return MainAttackIcon;
		case AbilityAction::Ability1:
			return Ability1Icon;
		case AbilityAction::Ability2:
			return Ability2Icon;
		case AbilityAction::Ability3:
			return Ability3Icon;
		case AbilityAction::Ultimate:
			return UltimateIcon;
	}
}
