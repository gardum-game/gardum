/*
 *  Copyright © 2021-2022 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with Gardum. If not, see <https://www.gnu.org/licenses/>.
 */

use std::any::type_name;

use bevy::{
    prelude::*,
    reflect::{
        serde::{ReflectDeserializer, ReflectSerializer},
        TypeRegistry,
    },
};
use clap::once_cell::sync::OnceCell;
use serde::{de::DeserializeSeed, Deserialize, Deserializer, Serialize, Serializer};

pub(super) struct ReflectObjectPlugin;

impl Plugin for ReflectObjectPlugin {
    fn build(&self, app: &mut App) {
        // Since the tests run in parallel and share global storage, we turn off the panic if the global
        // [`TypeRegistry`] has been initialized and make sure it contains at least [`Transform`] with its types.
        #[cfg(test)]
        app.register_type::<Transform>()
            .register_type::<Vec3>()
            .register_type::<Quat>();

        TYPE_REGISTRY
            .set(app.world.resource::<TypeRegistry>().clone())
            .unwrap_or_else(|_| {
                if cfg!(not(test)) {
                    panic!(
                        "Global {} for {} should be initialized only once",
                        type_name::<TypeRegistry>(),
                        type_name::<ReflectObjectPlugin>()
                    );
                }
            });
    }
}

/// Store [`TypeRegistry`] globally to acess it from Serialize / Deserialize traits.
/// Copying is very cheap because [`TypeRegistry`] is reference-counted.
static TYPE_REGISTRY: OnceCell<TypeRegistry> = OnceCell::new();

/// Get reference to [`TypeRegistry`] from global storage
/// Panics if it wasn't initialized
fn global_type_registry() -> &'static TypeRegistry {
    match TYPE_REGISTRY.get() {
        Some(type_registry) => type_registry,
        None => panic!(
            "{} wasn't initialized, probably missing {}",
            type_name::<TypeRegistry>(),
            type_name::<ReflectObjectPlugin>()
        ),
    }
}

/// Netype to serialize [`Reflect`]
#[derive(Deref, DerefMut)]
pub(super) struct ReflectObject(Box<dyn Reflect>);

impl Serialize for ReflectObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let type_registry = global_type_registry().read();
        ReflectSerializer::new(self.as_ref(), &type_registry).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ReflectObject {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let type_registry = global_type_registry().read();
        ReflectDeserializer::new(&type_registry)
            .deserialize(deserializer)
            .map(|b| Self(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser_de() {
        let mut app = App::new();
        app.add_plugin(ReflectObjectPlugin);

        const TRANSFORM: Transform = Transform::identity();
        let reflected = ReflectObject(TRANSFORM.clone_value());
        let bytes = rmp_serde::to_vec(&reflected).expect("Unable to serialize");

        let deserialized: ReflectObject =
            rmp_serde::from_slice(&bytes).expect("Unable to deserialize");

        assert!(
            deserialized.reflect_partial_eq(reflected.as_ref()).unwrap(),
            "Deserialized value should be equal to the reflected"
        );
        assert!(
            deserialized.reflect_partial_eq(&TRANSFORM).unwrap(),
            "Deserialized value should be equal to the original value"
        );
    }
}
