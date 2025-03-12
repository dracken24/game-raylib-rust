use raylib::prelude::*;
mod core;
use crate::core::Core;

fn main()
{
    let mut core = Core::new();

    while !core.should_close()
    {
        let mut d = core.begin_drawing();

        d.clear_background(Color::LIGHTGRAY);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
