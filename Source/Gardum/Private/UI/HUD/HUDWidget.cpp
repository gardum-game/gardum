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
#include "Components/ProgressBar.h"
#include "Core/GardumAttributeSet.h"
#include "UI/HUD/AbilityIcon.h"
#include "AbilitySystemInterface.h"

void UHUDWidget::SetPawn(APawn* NewPawn)
{
	const FGameplayAttribute HealthAttribute = UGardumAttributeSet::GetHealthAttribute();

	// Cleanup previous connections
	if (AbilitySystem != nullptr)
	{
		AbilitySystem->GetGameplayAttributeValueChangeDelegate(HealthAttribute).RemoveAll(this);
		AbilitySystem = nullptr;
	}

	if (NewPawn == nullptr)
	{
		AbilitySystem = nullptr;
		return;
	}

 	auto* AbilityInterface = Cast<IAbilitySystemInterface>(NewPawn);
	if (!ensureAlwaysMsgf(AbilityInterface != nullptr, TEXT("Possessed pawn do not have IAbilitySystemInterface")))
	{
		return;
	}

	AbilitySystem = AbilityInterface->GetAbilitySystemComponent();
	if (!ensureAlwaysMsgf(AbilitySystem != nullptr, TEXT("Ability system component is null in posessed actor")))
	{
		return;
	}

	// Subscribe for health updates
	HealthBar->SetPercent(AbilitySystem->GetNumericAttribute(HealthAttribute) / AbilitySystem->GetNumericAttributeBase(HealthAttribute));
	AbilitySystem->GetGameplayAttributeValueChangeDelegate(HealthAttribute).AddWeakLambda(this, [this](const FOnAttributeChangeData& Data)
		{ HealthBar->SetPercent(HealthBar->Percent * Data.NewValue / Data.OldValue); });

	// Display abilities
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
