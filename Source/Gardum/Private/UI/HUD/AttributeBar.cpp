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

#include "UI/HUD/AttributeBar.h"

#include "AbilitySystemComponent.h"
#include "GameplayEffectTypes.h"

void UAttributeBar::SetAttribute(UAbilitySystemComponent* NewAbilitySystem, const FGameplayAttribute& Attribute)
{
	// Cleanup previous connection
	if (AbilitySystem != nullptr)
	{
		AbilitySystem->GetGameplayAttributeValueChangeDelegate(Attribute).RemoveAll(this);
	}

	AbilitySystem = NewAbilitySystem;
	if (AbilitySystem == nullptr)
	{
		return;
	}

	SetPercent(AbilitySystem->GetNumericAttribute(Attribute) / AbilitySystem->GetNumericAttributeBase(Attribute));
	AbilitySystem->GetGameplayAttributeValueChangeDelegate(Attribute).AddUObject(this, &UAttributeBar::OnAttributeChanged);
}

void UAttributeBar::OnAttributeChanged(const FOnAttributeChangeData& Data)
{

	if (Data.NewValue < 0)
	{
		SetPercent(0);
		return;
	}

	SetPercent(Percent * Data.NewValue / Data.OldValue);
}
