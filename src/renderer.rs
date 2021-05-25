use sdl2::render::{Texture, WindowCanvas};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use crate::entity::Entity;

pub fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture,
          entity: &Entity) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    //for e in entity_list.iter() {
        let screen_position = entity.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, entity.sprite.width(),
            entity.sprite.height());

        canvas.copy_ex(texture, entity.current_frame(), screen_rect, 0.0, None, entity.flip_frame_horizontal(), false)?;
    //}

    canvas.present();

    Ok(())
}
