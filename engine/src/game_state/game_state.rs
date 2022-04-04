use entity_arena::{EntityArena, IsEntity};
use protocol::entities::PlayerId;

enum Position {
    Hand,
    Board,
    Abyss,
}

struct CardEntity<T: IsEntity> {
    obj: T,
    position: Position,
}

#[derive(Debug)]
struct GameState {
    player_a_id: PlayerId,
    player_b_id: PlayerId,
    entity_arena: EntityArena,
}

impl GameState {
    fn new(player_a_id: PlayerId, player_b_id: PlayerId) -> Self {
        Self {
            player_a_id,
            player_b_id,
            entity_arena: EntityArena::new(),
        }
    }
}
