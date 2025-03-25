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

    // let mut square_pos: Vector2 = core.center_position;
    core.player.position.x -= 25.0;
    core.player.position.y -= 25.0;
    let color: Color = core.player.color;
    let mut player_pos = core.player.position;

    while !core.should_close()
    {
        let mut d = core.begin_drawing();

        d.clear_background(Color::LIGHTGRAY);
        d.draw_rectangle(
            player_pos.x as i32,
            player_pos.y as i32,
            50,
            50,
            color
        );

        if (d.is_key_down(KeyboardKey::KEY_W))
        {
            player_pos.y -= 4.0;
        }
        if (d.is_key_down(KeyboardKey::KEY_S))
        {
            player_pos.y += 4.0;
        }
        if (d.is_key_down(KeyboardKey::KEY_A))
        {
            player_pos.x -= 4.0;
        }
        if (d.is_key_down(KeyboardKey::KEY_D))
        {
            player_pos.x += 4.0;
        }
    }

    core.player.position = player_pos;
}
