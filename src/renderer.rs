use sdl2::{
    render::{Texture, WindowCanvas},
    pixels::Color,
    rect::{Point, Rect},
};
use specs::prelude::*;

use crate::components::*;

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
);

pub fn render(canvas: &mut WindowCanvas, background: Color, textures: &[Texture], data: SystemData)
              -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for (pos, sprite) in (&data.0, &data.1).join() {
        let current_frame = sprite.region;

        // Allow (0,0) to be screen center
        let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, current_frame.width(),
            current_frame.height());

        canvas.copy_ex(
            &textures[sprite.spritesheet],
            current_frame,
            screen_rect,
            0.0, // rotation angle
            None, // rotate around this center
            sprite.flip_horizontal,
            false,  // flip vertical
        )?;
    }

    canvas.present();

    Ok(())
}
