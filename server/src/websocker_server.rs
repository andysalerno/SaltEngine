use futures::{AsyncRead, AsyncWrite};
use salt_engine::{
    cards::*,
    game_state::{Deck, GameState, MakePlayerView, PlayerId, UnitCardInstance},
    id::Id,
};
use server::messages::{Connection, FromClient, FromServer, GameSession};
use smol::{lock::Mutex, Async};
use std::{collections::HashMap, net::TcpListener, sync::Arc, time::Duration};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
struct GlobalContext {
    pub waiting_players: Vec<PlayerId>,
    pub sessions: HashMap<PlayerId, GameSession>,
}

impl GlobalContext {
    fn new() -> Self {
        Self {
            waiting_players: Vec::new(),
            sessions: HashMap::new(),
        }
    }
}

type SharedContext = Arc<Mutex<GlobalContext>>;

async fn handle_connection<S>(mut connection: Connection<S>, context: SharedContext) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    // Send a hello
    let player_id = PlayerId::new();
    connection.send(FromServer::Hello(player_id)).await?;

    // Wait for a join game request
    match connection.recv::<FromClient>().await {
        Some(FromClient::JoinGame) => println!("Received join game request"),
        _ => panic!("unexpected response"),
    };

    // ??? let context = context.lock_arc().await; ???
    let mut ctx = context.lock().await;

    let session;
    if let Some(waiting_player) = ctx.waiting_players.pop() {
        // Another player is ready to go, so start a session.

        session = GameSession {
            session_id: Id::new(),
            player_a_id: waiting_player,
            player_b_id: player_id,
        };

        // Stamp the session into the global context
        ctx.sessions.insert(session.player_a_id, session.clone());
        ctx.sessions.insert(session.player_b_id, session.clone());
        drop(ctx);
    } else {
        // No one is ready, so put ourselves in the waiting list
        ctx.waiting_players.push(player_id);
        drop(ctx);

        // We're in the waiting list, now just need to wait for a session to start.
        let found_session = loop {
            println!("Still waiting for another player to join.");
            smol::Timer::after(Duration::from_secs(5)).await;

            let active_sessions = &context.lock().await.sessions;

            match active_sessions.get(&player_id) {
                Some(session) => break session.clone(),
                _ => continue,
            }
        };

        session = found_session;
    }

    let player_a = session.player_a_id;
    let player_b = session.player_b_id;

    let mut player_a_deck = {
        let cards: Vec<UnitCardInstance> = (0..8)
            .flat_map(|_| {
                let cards = vec![
                    RicketyCannon.make_instance(),
                    Pawn.make_instance(),
                    EmotionalSupportDog.make_instance(),
                    ReallyBigRock.make_instance(),
                    AttackDog.make_instance(),
                    SleepingDog.make_instance(),
                    PopcornVendor.make_instance(),
                    PriestOfTheLowland.make_instance(),
                    FraidyCat.make_instance(),
                    OutdoorCat.make_instance(),
                    IndoorCat.make_instance(),
                ];

                cards
            })
            .collect();

        Deck::new(cards)
    };

    let mut player_b_deck = {
        let cards: Vec<UnitCardInstance> = (0..8)
            .flat_map(|_| {
                let cards = vec![
                    RicketyCannon.make_instance(),
                    Pawn.make_instance(),
                    EmotionalSupportDog.make_instance(),
                    ReallyBigRock.make_instance(),
                    AttackDog.make_instance(),
                    SleepingDog.make_instance(),
                    PopcornVendor.make_instance(),
                    PriestOfTheLowland.make_instance(),
                    FraidyCat.make_instance(),
                    OutdoorCat.make_instance(),
                    IndoorCat.make_instance(),
                ];

                cards
            })
            .collect();

        Deck::new(cards)
    };

    player_a_deck.shuffle();
    player_b_deck.shuffle();

    // Send session
    println!("Returning session: {:?}", session);
    connection.send(FromServer::Session(session)).await?;

    // Send game state
    let game_state = GameState::initial_state(player_a, player_a_deck, player_b, player_b_deck);
    connection
        .send(FromServer::State(game_state.player_view(player_a)))
        .await?;

    println!("Connection closed.");
    Ok(())
}

pub fn run() -> Result<()> {
    smol::block_on(async {
        let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 9000))?;

        let context: SharedContext = Arc::new(Mutex::new(GlobalContext::new()));

        while let Ok((stream, addr)) = listener.accept().await {
            println!("New connection from {}", addr);

            let ws_stream = async_tungstenite::accept_async(stream).await?;
            let connection = Connection::new(ws_stream);

            let context = context.clone();

            smol::spawn(async {
                handle_connection(connection, context).await.unwrap();
            })
            .detach();
        }

        Ok(())
    })
}
