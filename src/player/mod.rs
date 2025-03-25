/* =============================================================================== */
/* ---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~--- */
/*               -------------------------------------------------                 */
/*                PROJET: Rust Game          PAR: Dracken24                        */
/*               -------------------------------------------------                 */
/*                CREATED: 25-3rd-2025                                             */
/*                MODIFIED BY: Dracken24                                           */
/*                LAST MODIFIED: 25-3rd-2025                                       */
/*               -------------------------------------------------                 */
/*                FILE: mod.rs                                                     */
/*               -------------------------------------------------                 */
/* ---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~---~--- */
/* =============================================================================== */

// Rend le contenu du fichier core.rs accessible
pub mod player;

// Réexporte la structure Core pour la rendre accessible directement
pub use self::player::Player;
