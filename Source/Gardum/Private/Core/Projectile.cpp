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

#include "Core/Projectile.h"

#include "AbilitySystemComponent.h"
#include "Components/SphereComponent.h"
#include "Heroes/Hero.h"
#include "GameFramework/ProjectileMovementComponent.h"
#include "NiagaraComponent.h"

AProjectile::AProjectile()
	: Niagara(CreateDefaultSubobject<UNiagaraComponent>("Niagara"))
	, Collision(CreateDefaultSubobject<USphereComponent>("Collision"))
	, ProjectileMovement(CreateDefaultSubobject<UProjectileMovementComponent>("ProjectileMovement"))
{
	RootComponent = Collision;
	ProjectileMovement->UpdatedComponent = Collision;
	Niagara->SetupAttachment(Collision);
}

void AProjectile::BeginPlay()
{
	Super::BeginPlay();
	OnActorBeginOverlap.AddDynamic(this, &AProjectile::OnProjectileBeginOverlap);
}

void AProjectile::SetDamageEffectSpecHandle(FGameplayEffectSpecHandle Handle)
{
	DamageEffectSpecHandle = MoveTemp(Handle);
}

void AProjectile::OnProjectileBeginOverlap([[maybe_unused]] AActor* OverlappedActor, AActor* OtherActor)
{
	if (GetInstigator() == OtherActor)
	{
		return;
	}

	if (GetLocalRole() == ROLE_Authority)
	{
		if (auto* HitHero = Cast<AHero>(OtherActor); HitHero != nullptr)
		{
			HitHero->GetAbilitySystemComponent()->ApplyGameplayEffectSpecToSelf(*DamageEffectSpecHandle.Data.Get());
		}
	}

	Destroy();
}
