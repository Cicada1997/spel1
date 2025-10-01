use std::time::Duration;

use sdl2::rect::Rect as SDL_rect;
use sdl2::render::Canvas as SDL_canvas;
use sdl2::video::Window as SDL_window;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event as SDL_event;
use sdl2::EventPump as SDL_EventPump;

use hecs::World as ECSWorld;
use hecs::Entity as EntityId;
use hecs::DynamicBundle;

use crate::components::{
    position::Position,
    direction::Direction,

    health::Health,
    visual::Visual,

    player::Player,
    enemy::Enemy,
};

use crate::game_events::{
    GameEvents,
    Event,
};

use crate::systems::{
    rendering,
};

pub struct Game {
    pub ecs:         ECSWorld,
    pub player:      EntityId,
    pub game_events: GameEvents,
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
                x:          0, 
                y:          0, 
            },
            Health {
                points:     100,
                max:        120,
                regen:      8,
            },
            Visual{},
        ));

        let mut game_events = GameEvents::new();

        Self {
            ecs:        ecs,
            player:     player_id,
            game_events: game_events,
        }
    }

    pub fn player_events(&mut self, event: SDL_event) {
        match event {
            SDL_event::KeyDown { keycode: Some(Keycode::W), .. } => {
                self.game_events.append(
                    Event::ENTITY_MOVEMENT(
                        self.player, 
                        Direction::UP
                    )
                );
            },
            SDL_event::KeyDown { keycode: Some(Keycode::A), .. } => {
                self.game_events.append(
                    Event::ENTITY_MOVEMENT(
                        self.player, 
                        Direction::LEFT
                    )
                );
            },
            SDL_event::KeyDown { keycode: Some(Keycode::S), .. } => {
                self.game_events.append(
                    Event::ENTITY_MOVEMENT(
                        self.player, 
                        Direction::DOWN
                    )
                );
            },
            SDL_event::KeyDown { keycode: Some(Keycode::D), .. } => {
                self.game_events.append(
                    Event::ENTITY_MOVEMENT(
                        self.player, 
                        Direction::RIGHT
                    )
                );
            },
            
            _ => {}
        }
    }

    pub fn update(&mut self, event_pump: &mut SDL_EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                SDL_event::Quit {..} |
                SDL_event::KeyDown { keycode: Some(Keycode::Escape), .. } => { return false },
                _ => {}
            }

            self.player_events(event);
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
