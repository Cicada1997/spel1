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

    let speed = 40;

    match dir {
        Direction::Up    => { pos.y -= speed },
        Direction::Down  => { pos.y += speed },
        Direction::Left  => { pos.x -= speed },
        Direction::Right => { pos.x += speed },
        _ => {}
    }
}
