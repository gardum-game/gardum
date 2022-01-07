/*
 *  Copyright © 2021-2022 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation; either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a get of the GNU Affero General Public License
 *  along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 */

pub mod north;

use bevy::prelude::*;
use strum::EnumIter;

use super::{ability::Abilities, CharacterBundle};
use crate::core::player::PlayerOwner;
use north::NorthPlugin;

pub struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(NorthPlugin);
    }
}

#[derive(Bundle)]
pub struct HeroBundle {
    player: PlayerOwner,
    kind: HeroKind,
    abilities: Abilities,

    #[bundle]
    character: CharacterBundle,
}

impl HeroBundle {
    pub fn hero(
        player: PlayerOwner,
        kind: HeroKind,
        transform: Transform,
        commands: &mut Commands,
        #[cfg(feature = "client")] meshes: &mut ResMut<Assets<Mesh>>,
        #[cfg(feature = "client")] materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        match kind {
            HeroKind::North => HeroBundle::north(
                player,
                transform,
                commands,
                #[cfg(feature = "client")]
                meshes,
                #[cfg(feature = "client")]
                materials,
            ),
        }
    }
}

#[derive(Clone, Copy, PartialEq, EnumIter, Debug)]
pub enum HeroKind {
    North,
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::CommandQueue;
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn hero_bundle() {
        let mut app = App::build().app;
        let player = app.world.spawn().id();
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);

        for expected_kind in HeroKind::iter() {
            for expected_translation in [Vec3::ZERO, Vec3::ONE] {
                let hero_bundle = HeroBundle::hero(
                    PlayerOwner(player),
                    expected_kind,
                    Transform::from_translation(expected_translation),
                    &mut commands,
                );

                assert_eq!(
                    hero_bundle.character.pbr.transform.translation, expected_translation,
                    "Translation should be equal to requested"
                );
                assert_eq!(
                    hero_bundle.kind, expected_kind,
                    "Hero kind should be equal to requested"
                );
            }
        }
    }
}
