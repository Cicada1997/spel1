use hecs::Entity as EntityId;

use crate::components::{
    position::Position,
    direction::Direction,
};

pub enum Event {
    ENTITY_MOVEMENT(EntityId, Direction),
    ENTITY_DEATH(EntityId, Position),
    ATTACK(i32, Position, i32), // @param (strength, position, range)
}

pub struct GameEvents {
    events: Vec<Event>,
}

impl GameEvents {
    pub fn new() -> Self {

        let events = Vec::<Event>::new();
        Self {
            events: events,
        }
    }

    pub fn append(&mut self, event: Event) {
        self.events.push(event);
    }
}
