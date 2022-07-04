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

use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use iyes_loopless::prelude::*;
use std::{any::TypeId, marker::PhantomData};

use super::{NetworkTick, Replication};

pub(super) struct ComponentReplicationPlugins;

// Registers types for replication
impl PluginGroup for ComponentReplicationPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(ComponentReplicationPlugin::<Transform>::default());
    }
}

/// Registers functions that track changes for the [`Component`] of type `T`.
#[derive(Default)]
struct ComponentReplicationPlugin<T: Component + Reflect> {
    component: PhantomData<T>,
}

impl<T: Component + Reflect> Plugin for ComponentReplicationPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            // TODO stageless: Add to the fixed timestep
            CoreStage::PostUpdate,
            Self::component_changes_system.run_if_resource_exists::<RenetServer>(),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            Self::component_removal_system.run_if_resource_exists::<RenetServer>(),
        );
    }
}

impl<T: Component + Reflect> ComponentReplicationPlugin<T> {
    fn component_changes_system(
        network_tick: Res<NetworkTick>,
        mut replicated_entities: Query<&mut Replication, Changed<T>>,
    ) {
        for mut replication in replicated_entities.iter_mut() {
            let change_ticks = replication.entry(TypeId::of::<T>()).or_default();
            change_ticks.changed = network_tick.0;
        }
    }

    fn component_removal_system(
        network_tick: Res<NetworkTick>,
        removals: RemovedComponents<T>,
        mut replicated_entities: Query<&mut Replication>,
    ) {
        for entity in removals.iter() {
            if let Ok(mut replication) = replicated_entities.get_mut(entity) {
                let change_ticks = replication.entry(TypeId::of::<T>()).or_default();
                change_ticks.removed = network_tick.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::network::tests::{NetworkPreset, TestNetworkPlugin};

    use super::*;

    #[test]
    fn updates_on_changes() {
        let mut app = App::new();
        app.add_plugin(TestNetworkPlugin::new(NetworkPreset::Server))
            .add_plugin(ComponentReplicationPlugin::<Transform>::default())
            .insert_resource(NetworkTick(10));
        let entity = app.world.spawn().insert(Replication::default()).id();

        for transform in [Transform::identity(), Transform::default()] {
            app.world.resource_mut::<NetworkTick>().0 += 1;
            app.world.entity_mut(entity).insert(transform);

            app.update();

            let replication = app.world.get::<Replication>(entity).unwrap();
            let change_ticks = *replication
                .get(&TypeId::of::<Transform>())
                .expect("Replication components should contain Transform");

            let mut network_tick = app.world.resource_mut::<NetworkTick>();
            assert_eq!(
                change_ticks.changed, network_tick.0,
                "Tick when resource changed should be equal to the current network tick"
            );
            assert!(
                change_ticks.removed < network_tick.0,
                "Tick when resource removed should be less then the current network tick"
            );

            network_tick.0 += 1;
            app.world.entity_mut(entity).remove::<Transform>();

            app.update();

            let replication = app.world.get::<Replication>(entity).unwrap();
            let change_ticks = replication
                .get(&TypeId::of::<Transform>())
                .expect("Replication components should contain Transform");

            let network_tick = app.world.resource::<NetworkTick>();
            assert_eq!(
                change_ticks.removed, network_tick.0,
                "Tick when resource removed should be equal to the current network tick"
            );
            assert!(
                change_ticks.changed < network_tick.0,
                "Tick when resource changed should be less then the current network tick"
            );
        }
    }
}
