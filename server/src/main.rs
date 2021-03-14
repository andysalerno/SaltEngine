mod matchmaking_queue;

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
    use crate::models::PlayerQueue;
    use salt_engine::{game_state::PlayerId, id::Id};
    use serde::Serialize;
    use std::convert::Infallible;

    #[derive(Serialize)]
    struct NewGameResponse {
        game_id: Id,
    }

    impl NewGameResponse {
        fn new(game_id: Id) -> Self {
            Self { game_id }
        }
    }

    pub async fn new_game(queue: PlayerQueue) -> Result<impl warp::Reply, Infallible> {
        // First, try to pop from the queue if possible.
        let mut queue = queue.lock().await;
        queue.push_front(PlayerId::new());
        let player_count = queue.len();

        let r = NewGameResponse::new(Id::new());
        let s = serde_json::to_string(&r).unwrap();

        let message = format!(
            "There are currently {} players in the queue. Your ID is: {}",
            player_count, s
        );
        Ok(message)
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
