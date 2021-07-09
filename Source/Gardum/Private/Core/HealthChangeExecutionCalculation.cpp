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

#include "Core/HealthChangeExecutionCalculation.h"
#include "Core/GardumAttributeSet.h"

UHealthChangeExecutionCalculation::UHealthChangeExecutionCalculation(const FObjectInitializer& ObjectInitializer)
	: UGameplayEffectExecutionCalculation(ObjectInitializer)
{
	DEFINE_ATTRIBUTE_CAPTUREDEF(UGardumAttributeSet, Health, Target, false);

	RelevantAttributesToCapture.Add(HealthDef);
}

void UHealthChangeExecutionCalculation::Execute_Implementation(const FGameplayEffectCustomExecutionParameters& ExecutionParams, FGameplayEffectCustomExecutionOutput& OutExecutionOutput) const
{
	float NewHealth = 0.0f;
	ExecutionParams.AttemptCalculateCapturedAttributeMagnitude(HealthDef, {}, NewHealth);
	OutExecutionOutput.AddOutputModifier(FGameplayModifierEvaluatedData(HealthProperty, EGameplayModOp::Override, NewHealth));
}
