use std::time::Duration;

use sdl2::rect::Rect as SDL_rect;
use sdl2::render::Canvas as SDL_canvas;
use sdl2::video::Window as SDL_window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use hecs::World as ECSWorld;
use hecs::Entity as EntityId;
use hecs::DynamicBundle;

use crate::components::{
    position::Position,
    health::Health,
    visual::Visual,
    player::Player,
    enemy::Enemy,
};

pub struct Game {
    ecs:        ECSWorld,
    player:     EntityId,
}

impl Game {
    pub fn new() -> Self {
        let mut ecs = ECSWorld::new();

        let mut player_id = ecs.spawn((
            Player {
                name:      "Theo",
                kill_count: 0
            },
            Position {
                x:      0, 
                y:      0, 
            },
            Health {
                points: 100,
                max: 120,
                regen: 8,
            },
            Visual{},
        ));

        Self {
            ecs: ecs,
            player: player_id,
        }
    }

    pub fn handle_events(&self) {

    }

    pub fn update(&self, event_pump: &mut EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false
                },
                _ => {}
            }
        }

        return true
    }

    // pub fn render(&self, canvas: &mut SDL_canvas<SDL_window>) {
    //     for (id, (position, visual)) in &mut self.ecs.query::<(&Position, &Visual)>() {
    //         canvas.set_draw_color(Color::RGB(0, 255, 0));
    //         let rect = SDL_rect::new(position.x, position.y, 50, 50);
    //         let _ = canvas.fill_rect(rect);
    //     }
    // }
}
