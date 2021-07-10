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

#include "UI/Scoreboard/ScoreboardEntry.h"

#include "Components/TextBlock.h"
#include "Core/GardumPlayerState.h"

void UScoreboardEntry::NativeOnListItemObjectSet(UObject* ListItemObject)
{
	IUserObjectListEntry::NativeOnListItemObjectSet(ListItemObject);

	if (!ensureAlwaysMsgf(ListItemObject != nullptr, TEXT("Received an invalid scoreboard entry")))
	{
		return;
	}

	// Unbind old delegates
	if (PlayerState != nullptr)
	{
		PlayerState->OnKill().RemoveAll(KillsText);
		PlayerState->OnDeath().RemoveAll(DeathsText);
		PlayerState->OnDamage().RemoveAll(DamageText);
		PlayerState->OnHealing().RemoveAll(HealingText);
	}

	PlayerState = Cast<AGardumPlayerState>(ListItemObject);
	if (!ensureAlwaysMsgf(PlayerState != nullptr, TEXT("Scoreboard entry do not inherit from the AGardumPlayerState")))
	{
		return;
	}

	PlayerText->SetText(FText::FromString(PlayerState->GetPlayerName()));
	KillsText->SetText(FText::AsNumber(PlayerState->GetKills()));
	DeathsText->SetText(FText::AsNumber(PlayerState->GetDeaths()));
	DamageText->SetText(FText::AsNumber(PlayerState->GetDamage()));
	HealingText->SetText(FText::AsNumber(PlayerState->GetHealing()));

	PlayerState->OnKill().AddWeakLambda(KillsText, [KillsText = KillsText](int16 Kills)
		{ KillsText->SetText(FText::AsNumber(Kills)); });
	PlayerState->OnDeath().AddWeakLambda(DeathsText, [DeathsText = DeathsText](uint16 Deaths)
		{ DeathsText->SetText(FText::AsNumber(Deaths)); });
	PlayerState->OnDamage().AddWeakLambda(DamageText, [DamageText = DamageText](uint32 Damage)
		{ DamageText->SetText(FText::AsNumber(Damage)); });
	PlayerState->OnHealing().AddWeakLambda(HealingText, [HealingText = HealingText](uint32 Healing)
		{ HealingText->SetText(FText::AsNumber(Healing)); });
}
