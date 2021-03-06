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

#include "Blueprint/IUserObjectListEntry.h"
#include "Blueprint/UserWidget.h"
#include "CoreMinimal.h"

#include "ScoreboardEntry.generated.h"

class UTextBlock;
class AGardumPlayerState;

UCLASS()
class GARDUM_API UScoreboardEntry : public UUserWidget, public IUserObjectListEntry
{
	GENERATED_BODY() // NOLINT

protected:
	void NativeOnListItemObjectSet(UObject* ListItemObject) override;

private:
	UPROPERTY(meta = (BindWidget))
	UTextBlock* PlayerText;

	UPROPERTY(meta = (BindWidget))
	UTextBlock* KillsText;

	UPROPERTY(meta = (BindWidget))
	UTextBlock* DeathsText;

	UPROPERTY(meta = (BindWidget))
	UTextBlock* DamageText;

	UPROPERTY(meta = (BindWidget))
	UTextBlock* HealingText;

	UPROPERTY()
	AGardumPlayerState* PlayerState;
};
