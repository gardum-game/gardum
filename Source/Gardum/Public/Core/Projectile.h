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
#include "GameFramework/Actor.h"
#include "GameplayEffectTypes.h"

#include "Projectile.generated.h"

class UProjectileMovementComponent;
class UNiagaraComponent;
class USphereComponent;

UCLASS()
class GARDUM_API AProjectile : public AActor
{
	GENERATED_BODY() // NOLINT

public:
	AProjectile();

	void BeginPlay() override;

	void SetDamageEffectSpecHandle(FGameplayEffectSpecHandle Handle);

private:
	UFUNCTION()
	void OnProjectileBeginOverlap(AActor* OverlappedActor, AActor* OtherActor);

	UPROPERTY(VisibleAnywhere, Category = "Components")
	UNiagaraComponent* Niagara;

	UPROPERTY(VisibleAnywhere, Category = "Components")
	TObjectPtr<USphereComponent> Collision;

	UPROPERTY(VisibleAnywhere, Category = "Components")
	UProjectileMovementComponent* ProjectileMovement;

	FGameplayEffectSpecHandle DamageEffectSpecHandle;
};
