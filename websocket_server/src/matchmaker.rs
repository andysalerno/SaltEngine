use crate::connection::Connection;
use log::info;
use smol::lock::Mutex;

#[derive(Default, Debug)]
pub struct MatchMaker {
    waiting_players: Mutex<Vec<Connection>>,
}

impl MatchMaker {
    /// Try to match the player with another player in the waitlist.
    /// If the match succeeds, returns Some(other_player_id).
    /// If it fails, we get put in the queue, and this returns None.
    pub async fn match_player(
        &self,
        player_connection: Connection,
    ) -> Option<(Connection, Connection)> {
        info!("Trying to match a player...");
        let mut waiting_players = self.waiting_players.lock().await;

        if let Some(other_player_connection) = waiting_players.pop() {
            // Found someone already waiting.
            info!("Found a player already waiting.");
            Some((player_connection, other_player_connection))
        } else {
            // No one else here. We'll queue up, and get signaled when someone else joins.
            info!("No player was found. Queueing up.");
            waiting_players.push(player_connection);
            None
        }
    }
}
