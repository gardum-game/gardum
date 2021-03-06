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

#include "CoreMinimal.h"
#include "GameFramework/HUD.h"

#include "GardumHUD.generated.h"

class UAbilitySystemComponent;
class UHUDWidget;
class UScoreboard;

UCLASS()
class GARDUM_API AGardumHUD : public AHUD
{
	GENERATED_BODY() // NOLINT

public:
	void PostInitializeComponents() override;
	void BeginPlay() override;

	void SetAbilitySystem(UAbilitySystemComponent* AbilitySystem);

private:
	UPROPERTY(EditAnywhere, Category = "HUD")
	TSubclassOf<UHUDWidget> HUDClass;

	UPROPERTY()
	UHUDWidget* HUD;

	UPROPERTY(EditAnywhere, Category = "HUD")
	TSubclassOf<UScoreboard> ScoreboardClass;

	UPROPERTY()
	UScoreboard* Scoreboard;
};
