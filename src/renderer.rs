use nalgebra::vector;
use sdl2::{
    gfx::primitives::DrawRenderer,
    render::{Texture, WindowCanvas},
    pixels::Color,
    //rect::{Point, Rect},
};
use specs::prelude::*;

use crate::components::*;

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, RenderableCircle>,
    ReadStorage<'a, RenderablePolygon>,
    ReadStorage<'a, RenderColour>,
);

pub fn render(canvas: &mut WindowCanvas, background: Color, _textures: &[Texture], data: SystemData)
              -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    // Transform positions such that the screen centre is (0, 0).
    let pos_transform = vector![(width / 2) as f64, (height / 2) as f64];

    // Render polygons.
    for (position, polygon, colour) in (&data.0, &data.2, &data.3).join() {
        let p = position.vector + pos_transform;

        canvas.filled_polygon(&polygon.vx(p),
                              &polygon.vy(p),
                              colour.sdl_colour())?;
    }

    // Render circles.
    for (position, circle, colour) in (&data.0, &data.1, &data.3).join() {
        let p = position.vector + pos_transform;

        canvas.filled_circle(p.x as i16,
                             p.y as i16,
                             circle.radius(),
                             colour.sdl_colour())?;
    }

    // Render lines.
    // TODO

    // Render sprites
    // TODO
    // old code below...
    //let current_frame = sprite.region;
    //
    //// Translate float coords to int based SDL point.
    //let raw_pos: Point = Point::new(pos.pos.x as i32, pos.pos.y as i32);
    //
    //// Translate coords such that (0,0) is the screen centre. SDL uses top left as (0,0).
    //let screen_position = raw_pos + Point::new(width as i32 / 2, height as i32 / 2);
    //let screen_rect = Rect::from_center(screen_position, current_frame.width(),
    //                                    current_frame.height());
    
    //canvas.copy_ex(
    //    &textures[sprite.spritesheet],
    //    current_frame,
    //    screen_rect,
    //    0.0, // rotation angle (unused)
    //    None, // rotate around this center (unused)
    //    sprite.flip_horizontal,
    //    false,  // flip vertical (unused)
    //)?;

    canvas.present();

    Ok(())
}
