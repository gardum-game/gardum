/*
 *  Copyright © 2021 Hennadii Chernyshchyk <genaloner@gmail.com>
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
use bevy_rapier3d::prelude::{
    ColliderBundle, ColliderShape, RigidBodyBundle, RigidBodyPositionSync, RigidBodyType,
};

use super::Authority;
use crate::characters::{CharacterBundle, HeroAssets};
use crate::core::{cli::Opts, AppState};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(start_session_system.system())
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(create_world_system.system())
                    .with_system(cursor_grab_system.system()),
            );
    }
}

fn start_session_system(opts: Res<Opts>, mut app_state: ResMut<State<AppState>>) {
    if opts.subcommand.is_some() {
        app_state.set(AppState::InGame).unwrap();
    }
}

fn create_world_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hero_assets: Res<HeroAssets>,
) {
    // Plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec3::new(4.0, 0.0, 4.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(10.0, 0.1, 10.0),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete);

    // Light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    let mut character = CharacterBundle::dummy(&hero_assets);
    character.rigid_body.position = Vec3::new(5.0, 10.0, 5.0).into();

    commands.spawn_bundle(character).insert(Authority);
}

fn cursor_grab_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}
