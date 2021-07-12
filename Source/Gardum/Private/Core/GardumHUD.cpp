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

#include "Core/GardumHUD.h"

#include "Core/GardumGameState.h"
#include "Core/GardumPlayerController.h"
#include "UI/HUD/HUDWidget.h"
#include "UI/Scoreboard/Scoreboard.h"

void AGardumHUD::PostInitializeComponents()
{
	Super::PostInitializeComponents();

	if (!GetWorld()->IsGameWorld())
	{
		return;
	}

	if (ensureAlwaysMsgf(HUDClass != nullptr, TEXT("HUD widget class is not specified")))
	{
		HUD = CreateWidget<UHUDWidget>(PlayerOwner.Get(), HUDClass);
	}

	if (ensureAlwaysMsgf(ScoreboardClass != nullptr, TEXT("Scoreboard widget class is not specified")))
	{
		Scoreboard = CreateWidget<UScoreboard>(PlayerOwner.Get(), ScoreboardClass);
	}
}

void AGardumHUD::BeginPlay()
{
	if (HUD != nullptr)
	{
		HUD->AddToViewport();
	}

	if (Scoreboard != nullptr)
	{
		Scoreboard->AddToViewport();

		auto* GameState = GetWorld()->GetGameState<AGardumGameState>();
		GameState->OnPlayerStateAdded().AddUObject(Scoreboard, &UScoreboard::AddPlayerState);
		GameState->OnPlayerStateRemoved().AddUObject(Scoreboard, &UScoreboard::RemovePlayerState);
		for (APlayerState *PlayerState : GameState->PlayerArray)
		{
			Scoreboard->AddPlayerState(PlayerState);
		}
	}
}

void AGardumHUD::SetAbilitySystem(UAbilitySystemComponent* AbilitySystem)
{
	HUD->SetAbilitySystem(AbilitySystem);
}
