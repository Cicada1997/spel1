extern crate sdl2;
extern crate hecs;

use std::time::Duration;

use sdl2::rect::Rect as SDL_rect;
use sdl2::render::Canvas as SDL_canvas;
use sdl2::video::Window as SDL_window;
use sdl2::pixels::Color;
use sdl2::event::Event as SDL_event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump as SDL_eventPump;

use hecs::World as ECSWorld;
use hecs::Entity as EntityId;
use hecs::DynamicBundle;

pub mod components;
pub mod systems;

pub mod game_events;
pub mod game_event_handler;

pub mod game;
pub use game::Game;

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

    let mut game = Game::new();


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
