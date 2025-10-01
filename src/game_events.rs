use hecs::Entity as EntityId;

use crate::components::{
    position::Position,
    direction::Direction,
};

pub enum EntityEvent {
    Move(Direction),
    Attack(),
    Death(),
    Heal()
}

pub enum GameEvent {
    EntityEvent(EntityId, EntityEvent)
}

// pub enum Event {
//     EntityMovement(EntityId, Direction),
//     EntityDeath(EntityId, Position),
//     Attack(EntityId), // @context obtain strength, position and other values 
//                       // to calculate damage and target.
//
//     // Attack(i32, Position, i32), // @param (strength, position, range)
// }

pub struct GameEvents {
    events: Vec<GameEvent>,
}

impl GameEvents {
    pub fn new() -> Self {

        let events = Vec::new();
        Self {
            events: events,
        }
    }

    pub fn append(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    pub fn iter(&mut self) -> &mut Vec<GameEvent> {
        return &mut self.events
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
