// use super::Game;
use crate::SDL_canvas;
use crate::SDL_window;
use crate::SDL_rect;
use crate::Color;

use crate::components::{
    position::Position,
    visual::Visual,
};

use crate::game::Game;


impl Game {
    pub fn render(&self, canvas: &mut SDL_canvas<SDL_window>) {
        for (id, (position, visual)) in &mut self.ecs.query::<(&Position, &Visual)>() {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            let rect = SDL_rect::new(position.x, position.y, 50, 50);
            let _ = canvas.fill_rect(rect);
        }
    }
}
