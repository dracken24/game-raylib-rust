/* =============================================================================== */
/* ---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~--- */
/*               -------------------------------------------------                 */
/*                PROJET: Java Dev          PAR: Dracken24                         */
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

pub struct Core
{
    pub rayLib: RaylibHandle,
    pub thread: RaylibThread,
}

impl Core
{
    pub fn new() -> Core
    {
        let (rl, thread) = raylib::init()
            .size(1200, 750)
            .title("Untitled RPG Game")
            .build();

        Core
        {
            rayLib: rl,
            thread: thread,
        }
    }

    // Ajout d'une méthode pour vérifier si la fenêtre doit se fermer
    pub fn should_close(&self) -> bool
    {
        self.rayLib.window_should_close()
    }

    // Méthode pour commencer le dessin
    pub fn begin_drawing(&mut self) -> RaylibDrawHandle
    {
        self.rayLib.begin_drawing(&self.thread)
    }
}