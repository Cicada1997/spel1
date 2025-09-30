extern crate sdl2;
extern crate hecs;

use std::time::Duration;

use sdl2::rect::Rect as SDL_rect;
use sdl2::render::Canvas as SDL_canvas;
use sdl2::video::Window as SDL_window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump as SDL_eventPump;

use hecs::World as ECSWorld;
use hecs::Entity as EntityId;
use hecs::DynamicBundle;

pub mod components;
pub use crate::components::{
    position::Position,
    health::Health,
    visual::Visual,
    player::Player,
    enemy::Enemy,
};

pub mod event_loop;
pub use event_loop::EventLoop;

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

    pub fn update(&self, event_pump: &mut SDL_eventPump) -> bool {
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

    pub fn render(&self, canvas: &mut SDL_canvas<SDL_window>) {
        for (id, (position, visual)) in &mut self.ecs.query::<(&Position, &Visual)>() {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            let rect = SDL_rect::new(position.x, position.y, 50, 50);
            let _ = canvas.fill_rect(rect);
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("the gaming game", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let game = Game::new();


    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        if !game.update(&mut event_pump) {
            break 'running;
        }

        game.render(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
