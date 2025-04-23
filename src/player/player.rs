/* =============================================================================== */
/* ---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~--- */
/*               -------------------------------------------------                 */
/*                PROJET: Rust Game          PAR: Dracken24                        */
/*               -------------------------------------------------                 */
/*                CREATED: 25-3rd-2025                                             */
/*                MODIFIED BY: Dracken24                                           */
/*                LAST MODIFIED: 25-3rd-2025                                       */
/*               -------------------------------------------------                 */
/*                FILE: player.rs                                                  */
/*               -------------------------------------------------                 */
/* ---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~--- */
/* =============================================================================== */

#[path = "spritesheet.rs"] mod spritesheet;
use spritesheet::*;

use raylib::prelude::*;

pub struct Player
{
    pub position: Vector2,
    pub size: Vector2,
    pub color: Color,

    // pub idle_front: Spritesheet
}

impl Player
{
    pub fn new(pos: Vector2) -> Player
    {
        let siz: Vector2 = Vector2{x: 50.0, y: 50.0};

        return Player{
            position: pos,
            size: siz,
            color: raylib::color::Color::BLUE,
            // idle_front: Spritesheet::new(rl, thread, "assets/player/idle_front.png", 4)
        }
    }

/*************************************************************************************/
/***                                    Setters                                    ***/
/*************************************************************************************/

    pub fn set_position(&mut self, pos: Vector2)
    {
        self.position = pos;
    }

/*************************************************************************************/
/***                                    Getters                                    ***/
/*************************************************************************************/

    pub fn get_position(&self) -> Vector2
    {
        return self.position;
    }
}
