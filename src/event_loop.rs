use hecs::Entity as EntityId;

use crate::components::{
    position::Position,
    direction::Direction,
};

pub enum Event {
    ENTITY_MOVEMENT(EntityId, Direction),
    ENTITY_DEATH(EntityId, Position),
}

pub struct EventLoop {
    events: Vec<Event>,
}
