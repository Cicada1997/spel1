use crate::game::Game;

use crate::game_events::{
    GameEvents,
    GameEvent,
    EntityEvent,
};

use crate::systems::{
    rendering,
    entity_movement::entity_movement,
};

impl Game {
    pub fn handle_events(&mut self) {
        for event in self.game_events.iter() {
            match event {
                GameEvent::EntityEvent(eid, EntityEvent::Move(dir)) => {
                    entity_movement(&mut self.ecs, *eid, dir);
                }

                _ => {}
            }
        }

        self.game_events.clear();
    }
}
