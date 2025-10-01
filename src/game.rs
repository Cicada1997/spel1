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

use crate::game_event_handler;

use crate::game_events::{
    GameEvents,
    GameEvent,
    EntityEvent,
};

use crate::systems::{
    rendering,
    entity_movement,
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
            ecs:         ecs,
            player:      player_id,
            game_events: game_events,
        }
    }

    pub fn player_events(&mut self, keys: Vec<Keycode>) {
        if keys.contains(&Keycode::W) {
            self.game_events.append(
                GameEvent::EntityEvent(
                    self.player, 
                    EntityEvent::Move(
                        Direction::Up
                    )
                )
            );
        }

        if keys.contains(&Keycode::A) {
            self.game_events.append(
                GameEvent::EntityEvent(
                    self.player, 
                    EntityEvent::Move(
                        Direction::Left
                    )
                )
            );
        }


        if keys.contains(&Keycode::S) {
            self.game_events.append(
                GameEvent::EntityEvent(
                    self.player, 
                    EntityEvent::Move(
                        Direction::Down
                    )
                )
            );
        }

        if keys.contains(&Keycode::D) {
            self.game_events.append(
                GameEvent::EntityEvent(
                    self.player, 
                    EntityEvent::Move(
                        Direction::Right
                    )
                )
            );
        }
    }

    pub fn update(&mut self, event_pump: &mut SDL_EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                SDL_event::Quit {..} |
                SDL_event::KeyDown { keycode: Some(Keycode::Escape), .. } => { return false },
                _ => {}
            }
        }

        let keys = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        self.player_events(keys);


        self.handle_events();

        return true
    }
}
