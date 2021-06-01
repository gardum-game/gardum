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

#include "AttributeSet.h"
#include "CoreMinimal.h"

#include "GardumAttributeSet.generated.h"

UCLASS()
class GARDUM_API UGardumAttributeSet : public UAttributeSet
{
	GENERATED_BODY() // NOLINT

public:
	UGardumAttributeSet() = default;

	GAMEPLAYATTRIBUTE_PROPERTY_GETTER(UGardumAttributeSet, Health);

private:
	UPROPERTY(EditAnywhere, ReplicatedUsing = OnRep_Health)
	FGameplayAttributeData Health;

	UFUNCTION()
	virtual void OnRep_Health(const FGameplayAttributeData& OldHealth);
};
