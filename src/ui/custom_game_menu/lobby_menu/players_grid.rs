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

use bevy::core::Name;
use bevy_egui::egui::{Grid, Ui};
use std::iter;

pub(super) struct PlayersGrid<'a, T: Iterator<Item = &'a Name>> {
    players: T,
    slots_count: u8,
}

impl<'a, T: Iterator<Item = &'a Name>> PlayersGrid<'a, T> {
    pub(super) fn new(players: T, slots_count: u8) -> Self {
        Self {
            players,
            slots_count,
        }
    }
}

impl<'a, T: Iterator<Item = &'a Name>> PlayersGrid<'a, T> {
    pub(super) fn show(self, ui: &mut Ui) {
        Grid::new("Players grid").striped(true).show(ui, |ui| {
            ui.heading("Players");
            ui.end_row();
            for (text, index) in self
                .players
                .map(|player| player.as_str())
                .chain(iter::repeat(Default::default()))
                .zip(0..self.slots_count)
            {
                ui.label(format!("{}. {}", index + 1, text));
                ui.end_row();
            }
        });
    }
}
