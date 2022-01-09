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

use bevy::prelude::*;
use heron::{CollisionShape, RigidBody};

use super::Authority;
use crate::{
    characters::heroes::{HeroKind, HeroSelectEvent},
    core::{cli::Opts, AppState},
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_session_system)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(create_world_system));
    }
}

fn start_session_system(opts: Res<Opts>, mut app_state: ResMut<State<AppState>>) {
    if opts.subcommand.is_some() {
        app_state.set(AppState::InGame).unwrap();
    }
}

fn create_world_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Authority>>,
    mut hero_spawn_events: EventWriter<HeroSelectEvent>,
    #[cfg(feature = "client")] mut meshes: ResMut<Assets<Mesh>>,
    #[cfg(feature = "client")] mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    commands
        .spawn_bundle(PbrBundle {
            #[cfg(feature = "client")]
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            #[cfg(feature = "client")]
            material: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
            ..Default::default()
        })
        .insert(Transform::from_translation(Vec3::new(4.0, 0.0, 4.0)))
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(10.0, 0.1, 10.0),
            border_radius: None,
        });

    // Light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    hero_spawn_events.send(HeroSelectEvent {
        player: player_query.single(),
        kind: HeroKind::North,
        transform: Transform::from_translation(Vec3::new(5.0, 15.0, 5.0)),
    })
}
