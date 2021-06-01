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

        // Translate float coords to int based SDL point.
        let raw_pos: Point = Point::new(pos.x as i32, pos.y as i32);

        // Translate coords such that (0,0) is the screen centre. SDL uses top left as (0,0).
        let screen_position = raw_pos + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, current_frame.width(),
                                            current_frame.height());

        canvas.copy_ex(
            &textures[sprite.spritesheet],
            current_frame,
            screen_rect,
            0.0, // rotation angle (unused)
            None, // rotate around this center (unused)
            sprite.flip_horizontal,
            false,  // flip vertical (unused)
        )?;
    }

    canvas.present();

    Ok(())
}
