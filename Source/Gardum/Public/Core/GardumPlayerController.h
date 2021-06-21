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
#include "Delegates/DelegateCombinations.h"
#include "GameFramework/PlayerController.h"

#include "GardumPlayerController.generated.h"

UCLASS()
class GARDUM_API AGardumPlayerController : public APlayerController
{
	GENERATED_BODY() // NOLINT
	DECLARE_EVENT_OneParam(AGardumPlayerController, FOnPawnChanged, APawn*);

public:
	void SetPawn(APawn* InPawn) override;

	FOnPawnChanged& OnPawnChanged();

private:
	/** Unlike AController::OnNewPawn will be called on the client and on the server */
	FOnPawnChanged PawnChangedEvent;
};
