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

#include "Core/GardumPlayerController.h"

#include "AbilitySystemInterface.h"
#include "Core/GardumHUD.h"
#include "UI/HUD/HUDWidget.h"

void AGardumPlayerController::AcknowledgePossession(APawn* InPawn)
{
	Super::AcknowledgePossession(InPawn);

	auto* HUD = GetHUD<AGardumHUD>();
	if (ensureMsgf(HUD != nullptr, TEXT("AGardumPlayerController should have HUD of type AGardumHUD")))
	{
		if (auto* AbilityInterface = Cast<IAbilitySystemInterface>(InPawn); ensureAlwaysMsgf(AbilityInterface != nullptr, TEXT("Posessed pawn do not have ability system interface")))
		{
			HUD->GetHUDWidget()->SetAbilitySystem(AbilityInterface->GetAbilitySystemComponent());
		}
	}
}
