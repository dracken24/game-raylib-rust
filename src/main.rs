/* =============================================================================== */
/* ---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~--- */
/*               -------------------------------------------------                 */
/*                PROJET: Rust Game          PAR: Dracken24                        */
/*               -------------------------------------------------                 */
/*                CREATED: 12-3rd-2025                                             */
/*                MODIFIED BY: Dracken24                                           */
/*                LAST MODIFIED: 12-3rd-2025                                       */
/*               -------------------------------------------------                 */
/*                FILE: main.rs                                                    */
/*               -------------------------------------------------                 */
/* ---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~--- */
/* =============================================================================== */

#![allow(unused_parens)]
use raylib::prelude::*;
mod core;
use crate::core::Core;

fn main()
{
    let mut core = Core::new();

    core.raylib.set_target_fps(60);

    let mut square_pos: Vector2 = core.center_position;
    square_pos.x -= 25 as f32;
    square_pos.y -= 25 as f32;

    while !core.should_close()
    {

        let mut d = core.begin_drawing();

        d.clear_background(Color::LIGHTGRAY);
        d.draw_rectangle(
            square_pos.x as i32,
            square_pos.y as i32,
            50,
            50,
            Color::PURPLE
        );

        if (d.is_key_down(KeyboardKey::KEY_W))
        {
            square_pos.y -= 4 as f32;
        }
        if (d.is_key_down(KeyboardKey::KEY_S))
        {
            square_pos.y += 4 as f32;
        }
        if (d.is_key_down(KeyboardKey::KEY_A))
        {
            square_pos.x -= 4 as f32;
        }
        if (d.is_key_down(KeyboardKey::KEY_D))
        {
            square_pos.x += 4 as f32;
        }
    }
}
