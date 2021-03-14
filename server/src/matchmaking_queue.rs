use salt_engine::game_state::PlayerId;
use std::collections::VecDeque;

struct MatchmakingQueue(VecDeque<PlayerId>);

impl MatchmakingQueue {
    fn try_pop_player(&mut self) -> Option<PlayerId> {
        self.0.pop_back()
    }

    fn push_player(&mut self, player_id: PlayerId) {
        self.0.push_front(player_id);
    }
}
