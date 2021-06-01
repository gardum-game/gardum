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

#include "AbilitySystemComponent.h"
#include "AbilitySystemInterface.h"
#include "Core/GardumAttributeSet.h"
#include "Core/GardumPlayerController.h"
#include "UI/HUD/HeroHUD.h"

void AGardumHUD::PostInitializeComponents()
{
	Super::PostInitializeComponents();

	if (!ensureAlwaysMsgf(HUDClass != nullptr, TEXT("HUD widget is not specified")))
	{
		return;
	}

	HUD = CreateWidget<UHeroHUD>(PlayerOwner.Get(), HUDClass);
	HUD->AddToViewport();

	auto* PlayerController = Cast<AGardumPlayerController>(PlayerOwner);
	if (!ensureAlwaysMsgf(PlayerController != nullptr, TEXT("HUD have wrong controller type")))
	{
		return;
	}
	PlayerController->OnPawnChanged().AddUObject(this, &AGardumHUD::OnNewPawn);
}

void AGardumHUD::OnNewPawn(APawn* NewPawn)
{
	// Cleanup previous connections
	if (AbilitySystem != nullptr)
	{
		AbilitySystem->GetGameplayAttributeValueChangeDelegate(UGardumAttributeSet::GetHealthAttribute()).RemoveAll(HUD);
		AbilitySystem = nullptr;
	}

	auto* AbilityInterface = Cast<IAbilitySystemInterface>(NewPawn);
	if (AbilityInterface == nullptr)
	{
		return;
	}

	AbilitySystem = AbilityInterface->GetAbilitySystemComponent();
	if (!ensureAlwaysMsgf(AbilitySystem != nullptr, TEXT("Ability system component is null in posessed actor")))
	{
		return;
	}

	const FGameplayAttribute HealthAttribute = UGardumAttributeSet::GetHealthAttribute();
	HUD->SetHealth(AbilitySystem->GetNumericAttribute(HealthAttribute), AbilitySystem->GetNumericAttributeBase(HealthAttribute));
	AbilitySystem->GetGameplayAttributeValueChangeDelegate(HealthAttribute).AddUObject(HUD, &UHeroHUD::OnHealthAttributeChanged);
}
