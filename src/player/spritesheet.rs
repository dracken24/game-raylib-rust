/* =============================================================================== */
/* ---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~--- */
/*               -------------------------------------------------                 */
/*                PROJET: Rust Game          PAR: Dracken24                        */
/*               -------------------------------------------------                 */
/*                CREATED: 25-3rd-2025                                             */
/*                MODIFIED BY: Dracken24                                           */
/*                LAST MODIFIED: 25-3rd-2025                                       */
/*               -------------------------------------------------                 */
/*                FILE: spritesheet.rs                                             */
/*               -------------------------------------------------                 */
/* ---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~--- */
/* =============================================================================== */

use raylib::prelude::*;

pub struct Spritesheet
{
    pub texture: Texture2D,
    pub size_total: Vector2,
    pub size_view: Vector2
}

impl Spritesheet
{
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, texture_path: &str, nbr_frames: i32) -> Spritesheet
    {
        let tex: Texture2D = rl.load_texture(thread, texture_path).expect("Échec du chargement de la texture");
        let size: Vector2 =  Vector2{x: tex.width() as f32, y: tex.height() as f32};

        return Spritesheet{
            texture: tex,
            size_total: size,
            size_view: Vector2{x: size.x / nbr_frames as f32, y: size.y as f32}
        }
    }
}
