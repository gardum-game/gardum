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

#include "Heroes/North/FrostBolt.h"

#include "Core/Projectile.h"
#include "GameFramework/Character.h"

void UFrostBolt::ActivateAbility(const FGameplayAbilitySpecHandle Handle, const FGameplayAbilityActorInfo* ActorInfo, const FGameplayAbilityActivationInfo ActivationInfo, [[maybe_unused]] const FGameplayEventData* TriggerEventData)
{
	if (!HasAuthorityOrPredictionKey(ActorInfo, &ActivationInfo))
	{
		return;
	}

	if (!CommitAbility(Handle, ActorInfo, ActivationInfo))
	{
		EndAbility(CurrentSpecHandle, CurrentActorInfo, CurrentActivationInfo, true, true);
		return;
	}

	auto* Character = Cast<ACharacter>(GetAvatarActorFromActorInfo());
	if (!ensureMsgf(Character != nullptr, TEXT("Unable to get actor from the ability")))
	{
		ensureMsgf(GetAvatarActorFromActorInfo() != nullptr, TEXT("Unable to get any actor from the ability"));
		return;
	}

	FTransform MuzzleTransform = Character->GetMesh()->GetSocketTransform(AttachedSocketName);
	MuzzleTransform.SetRotation(Character->GetBaseAimRotation().Quaternion());

	auto* Projectile = GetWorld()->SpawnActorDeferred<AProjectile>(ProjectileClass, MuzzleTransform, GetOwningActorFromActorInfo(),
		Character, ESpawnActorCollisionHandlingMethod::AlwaysSpawn);

	Projectile->SetDamageEffectSpecHandle(MakeOutgoingGameplayEffectSpec(DamageEffectClass));
	Projectile->FinishSpawning(MuzzleTransform);

	EndAbility(CurrentSpecHandle, CurrentActorInfo, CurrentActivationInfo, true, false);
}
