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

mod dummy;

use bevy::prelude::*;

use super::{ability::Abilities, CharacterBundle};
use dummy::DummyPlugin;

pub struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<HeroSpawnEvent>().add_plugin(DummyPlugin);
    }
}

#[derive(Bundle)]
struct HeroBundle {
    abilities: Abilities,
    hero: Hero,

    #[bundle]
    character: CharacterBundle,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Hero {
    Dummy,
}

pub struct HeroSpawnEvent {
    pub hero: Hero,
    pub transform: Transform,
    pub authority: bool,
}
