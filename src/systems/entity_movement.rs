use hecs::World as ECSWorld;
use crate::hecs::Query;

use crate::EntityId;

use crate::components::{
    direction::Direction,
    position::Position,
};

use crate::game_events::{
    GameEvents,
    GameEvent,
    EntityEvent,
};

pub fn entity_movement(ecs: &mut ECSWorld, eid: EntityId, dir: &mut Direction) {
    let mut pos = ecs.query_one_mut::<(&mut Position)>(eid).unwrap();

    let speed = 5.0;

    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    match dir {
        Direction::Up    => { y -= speed },
        Direction::Down  => { y += speed },
        Direction::Left  => { x -= speed },
        Direction::Right => { x += speed },
        _ => {}
    }

    // if (x < 0.0 || x > 0.0) && (y < 0.0 && y > 0.0) {
    //     let  m = (x*x + y*y).sqrt();
    //     x /= m;
    //     y /= m;
    // }
    //
    // println!("x_vel: {x}, y_vel: {y}");

    pos.x += x as i32; pos.y += y as i32;
}
