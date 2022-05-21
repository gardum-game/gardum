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

use bevy_egui::egui::{
    epaint::{Vertex, WHITE_UV},
    *,
};

use crate::core::cooldown::Cooldown;

/// Displays ability icon and its cooldown.
pub(super) struct AbilityIcon<'a> {
    texture_id: TextureId,
    cooldown: Option<&'a Cooldown>,
}

impl<'a> AbilityIcon<'a> {
    /// `current` shouldn't be bigger then `max`
    pub(super) fn new(texture_id: TextureId, cooldown: Option<&'a Cooldown>) -> Self {
        Self {
            cooldown,
            texture_id,
        }
    }
}

impl Widget for AbilityIcon<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let AbilityIcon {
            texture_id,
            cooldown,
        } = self;
        let image = Image::new(texture_id, [64.0, 64.0]);

        let (rect, response) = ui.allocate_at_least(
            image.size(),
            Sense {
                click: false,
                drag: false,
                focusable: false,
            },
        );

        image.paint_at(ui, rect);

        if let Some(cooldown) = cooldown {
            let display_sec = (cooldown.duration().as_secs_f32() - cooldown.elapsed_secs()).ceil();
            if display_sec != 0.0 {
                ui.painter().add(Shape::mesh(generate_cooldown_mesh(
                    cooldown.percent_left(),
                    rect,
                )));

                let text: WidgetText = RichText::new(display_sec.to_string())
                    .font(FontId::monospace(25.0))
                    .strong()
                    .color(Color32::DARK_GRAY)
                    .into();
                let text_galley = text.into_galley(ui, None, f32::INFINITY, TextStyle::Heading);
                let text_pos = rect.center() - text_galley.size() / 2.0;
                ui.painter()
                    .with_clip_rect(rect)
                    .galley(text_pos, text_galley.galley);
            }
        }

        response
    }
}

/// Generates mesh for cooldown fade
///
/// `cooldown` should be from 0.0 to 1.0.
fn generate_cooldown_mesh(cooldown: f32, content_rect: Rect) -> Mesh {
    let segment_size = Vec2::new(content_rect.width() / 2.0, content_rect.height() / 2.0);
    let mut mesh = Mesh::default();

    /*
     * 2 1+9 8
     * 3  0  7
     * 4  5  6
     */
    add_vert(
        &mut mesh,
        content_rect.min.x + segment_size.x,
        content_rect.min.y + segment_size.y,
    );
    add_vert(
        &mut mesh,
        content_rect.min.x + segment_size.x,
        content_rect.min.y,
    );
    add_vert(&mut mesh, content_rect.min.x, content_rect.min.y);
    add_vert(
        &mut mesh,
        content_rect.min.x,
        content_rect.min.y + segment_size.y,
    );
    add_vert(&mut mesh, content_rect.min.x, content_rect.max.y);
    add_vert(
        &mut mesh,
        content_rect.min.x + segment_size.x,
        content_rect.max.y,
    );
    add_vert(&mut mesh, content_rect.max.x, content_rect.max.y);
    add_vert(
        &mut mesh,
        content_rect.max.x,
        content_rect.min.y + segment_size.y,
    );
    add_vert(&mut mesh, content_rect.max.x, content_rect.min.y);
    add_vert(
        &mut mesh,
        content_rect.min.x + segment_size.x,
        content_rect.min.y,
    );

    /*
     * Triangles:
     * _______
     * |\ | /|
     * |_\|/_|
     * | /|\ |
     * |/ | \|
     * -------
     */
    const TRIANGLES_COUNT: f32 = 8.0;
    let segments = cooldown * TRIANGLES_COUNT;
    let num_segments = segments.trunc() as u32;
    for segment_id in 0..num_segments {
        mesh.add_triangle(0, segment_id + 1, segment_id + 2);
    }

    let fract_segments = segments.fract();
    if fract_segments > 0.0 {
        if let (Some(vert_1), Some(vert_2)) = (
            mesh.vertices.get(num_segments as usize + 1).map(|x| x.pos),
            mesh.vertices.get(num_segments as usize + 2).map(|x| x.pos),
        ) {
            let vertex_id = add_vert(
                &mut mesh,
                (vert_2.x - vert_1.x) * fract_segments + vert_1.x,
                (vert_2.y - vert_1.y) * fract_segments + vert_1.y,
            );
            mesh.add_triangle(0, num_segments + 1, vertex_id);
        }
    }

    mesh
}

fn add_vert(mesh: &mut Mesh, x: f32, y: f32) -> u32 {
    let pos = mesh.vertices.len();
    mesh.vertices.push(Vertex {
        pos: Pos2::new(x, y),
        uv: WHITE_UV,
        color: Color32::from_rgba_unmultiplied(50, 50, 50, 150),
    });
    pos as u32
}
