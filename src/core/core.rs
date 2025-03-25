/* =============================================================================== */
/* ---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~--- */
/*               -------------------------------------------------                 */
/*                PROJET: Rust Game          PAR: Dracken24                        */
/*               -------------------------------------------------                 */
/*                CREATED: 12-3rd-2025                                             */
/*                MODIFIED BY: Dracken24                                           */
/*                LAST MODIFIED: 12-3rd-2025                                       */
/*               -------------------------------------------------                 */
/*                FILE: core.rs                                                    */
/*               -------------------------------------------------                 */
/* ---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~--- */
/* =============================================================================== */

use raylib::prelude::*;

#[path = "../player/player.rs"] mod player;
use player::*;

pub struct Core
{
    pub raylib: RaylibHandle,
    pub thread: RaylibThread,

    pub player: Player
}

impl Core
{
    pub fn new() -> Core
    {
        let (rl, thread) = raylib::init()
            .size(1200, 750)
            .title("Untitled RPG Game")
            .build();

        let center = Vector2 {
            x: rl.get_screen_width() as f32 / 2.0,
            y: rl.get_screen_height() as f32 / 2.0,
        };

        return Core 
        {
            raylib: rl,
            thread: thread,
            player: Player::new(center),
        }
    }

    // Ajout d'une méthode pour vérifier si la fenêtre doit se fermer
    pub fn should_close(&self) -> bool
    {
        return self.raylib.window_should_close()
    }

    // Méthode pour commencer le dessin
    pub fn begin_drawing(&mut self) -> RaylibDrawHandle
    {
        return self.raylib.begin_drawing(&self.thread)
    }
}
