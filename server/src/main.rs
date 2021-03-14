mod matchmaking_queue;
pub mod messages;

use models::{new_queue, GameQueue};
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let goodbye = warp::path!("goodbye" / String).map(|name| format!("goodbye, {}!", name));

    let player_queue = new_queue();

    let api = hello.or(goodbye).or(new_game(player_queue));

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}

pub fn new_game(
    queue: GameQueue,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("newgame")
        .and(warp::get())
        .and(with_queue(queue))
        .and_then(handlers::new_game)
}

pub fn game(
    queue: GameQueue,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("game" / String)
        .and(warp::get())
        .and(with_queue(queue))
        .and_then(handlers::game)
}

fn with_queue(
    queue: GameQueue,
) -> impl Filter<Extract = (GameQueue,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || queue.clone())
}

mod handlers {
    use crate::{messages::NewGameResponse, models::GameQueue};
    use salt_engine::id::Id;
    use std::convert::Infallible;

    pub async fn new_game(queue: GameQueue) -> Result<impl warp::Reply, Infallible> {
        let mut queue = queue.lock().await;
        if let Some(game_id) = queue.pop_back() {
            println!("Placing player in existing game: {:?}", game_id);
            let r = NewGameResponse::new(game_id);
            Ok(warp::reply::json(&r))
        } else {
            let game_id = Id::new();
            println!("Placing player in new game: {:?}", game_id);
            queue.push_front(game_id);
            let r = NewGameResponse::new(game_id);
            Ok(warp::reply::json(&r))
        }
    }

    pub async fn game(id: String, queue: GameQueue) -> Result<impl warp::Reply, Infallible> {
        let mut queue = queue.lock().await;
        if let Some(game_id) = queue.pop_back() {
            println!("Placing player in existing game: {:?}", game_id);
            let r = NewGameResponse::new(game_id);
            Ok(warp::reply::json(&r))
        } else {
            let game_id = Id::new();
            println!("Placing player in new game: {:?}", game_id);
            queue.push_front(game_id);
            let r = NewGameResponse::new(game_id);
            Ok(warp::reply::json(&r))
        }
    }
}

mod models {
    use salt_engine::id::Id;
    use std::{collections::VecDeque, sync::Arc};
    use tokio::sync::Mutex;

    /// So we don't have to tackle how different database work, we'll just use
    /// a simple in-memory DB, a vector synchronized by a mutex.
    pub type GameQueue = Arc<Mutex<VecDeque<Id>>>;

    pub fn new_queue() -> GameQueue {
        Arc::new(Mutex::new(VecDeque::new()))
    }
}
