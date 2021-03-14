mod matchmaking_queue;
pub mod messages;

use models::{new_queue, PlayerQueue};
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let goodbye = warp::path!("goodbye" / String).map(|name| format!("goodbye, {}!", name));

    let player_queue = new_queue();

    let api = hello.or(goodbye).or(new_game(player_queue));

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}

pub fn new_game(
    queue: PlayerQueue,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("newgame")
        .and(warp::get())
        .and(with_queue(queue))
        .and_then(handlers::new_game)
}

fn with_queue(
    queue: PlayerQueue,
) -> impl Filter<Extract = (PlayerQueue,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || queue.clone())
}

mod handlers {
    use crate::{messages::NewGameResponse, models::PlayerQueue};
    use salt_engine::{game_state::PlayerId, id::Id};
    use std::convert::Infallible;

    pub async fn new_game(queue: PlayerQueue) -> Result<impl warp::Reply, Infallible> {
        // First, try to pop from the queue if possible.
        let mut queue = queue.lock().await;
        queue.push_front(PlayerId::new());

        let mut r = NewGameResponse::new(Id::new());
        r.players_in_queue = queue.len();

        Ok(warp::reply::json(&r))
    }
}

mod models {
    use salt_engine::game_state::PlayerId;
    use std::{collections::VecDeque, sync::Arc};
    use tokio::sync::Mutex;

    /// So we don't have to tackle how different database work, we'll just use
    /// a simple in-memory DB, a vector synchronized by a mutex.
    pub type PlayerQueue = Arc<Mutex<VecDeque<PlayerId>>>;

    pub fn new_queue() -> PlayerQueue {
        Arc::new(Mutex::new(VecDeque::new()))
    }
}
