use crate::console_display::ConsoleDisplay;
use async_trait::async_trait;
use log::{error, info};
use protocol::{
    client_actions::{Attack, EndTurn, SummonCreatureFromHand},
    entities::{BoardPos, CreatureDefinition, PlayerHero, PlayerId, RowId},
    from_client::ClientAction,
    from_server::{EntityAdded, EntityUpdate, Notification},
};
use salt_engine::{
    game_agent::{ClientNotifier, GameClient, Prompter},
    game_state::{
        board::BoardView, GameStatePlayerView, GameStateView, HandView, IterAddons, IteratorAny,
        UnitCardInstancePlayerView,
    },
};
use smol::{
    channel::{Receiver, SendError, Sender},
    lock::Mutex,
    LocalExecutor,
};
use std::borrow::BorrowMut;
use std::{borrow::Borrow, collections::VecDeque, sync::Arc};
use thiserror::Error;
use websocket_client::local_state::LocalState;

#[derive(Debug, Error)]
enum ConsoleError {
    #[error("{0}")]
    UserInputError(String),
}

fn user_input_err<T: ToString>(msg: T) -> ConsoleError {
    ConsoleError::UserInputError(msg.to_string())
}

pub struct ConsoleAgent {
    id: PlayerId,
    local_state: Arc<Mutex<LocalState>>,
    notifier: Arc<ConsoleNotifier>,
    receiver: Receiver<Notification>, // not needed?
}

impl ConsoleAgent {
    pub fn new_with_id(my_id: PlayerId, opponent_id: PlayerId) -> Self {
        let local_state = Arc::new(Mutex::new(LocalState::new(my_id, opponent_id)));
        let (notifier, receiver) = ConsoleNotifier::new(Arc::clone(&local_state));
        Self {
            id: my_id,
            local_state,
            notifier: Arc::new(notifier),
            receiver,
        }
    }

    fn id(&self) -> PlayerId {
        self.id
    }
}

#[async_trait]
impl GameClient for ConsoleAgent {
    async fn make_prompter(&self) -> Arc<dyn Prompter> {
        Arc::new(ConsolePrompter::new(self.id()))
    }

    async fn make_notifier(&self) -> Arc<dyn ClientNotifier> {
        Arc::clone(&self.notifier) as Arc<dyn ClientNotifier>
    }

    async fn next_action(&mut self) -> ClientAction {
        let prompter = ConsolePrompter::new(self.id());

        let local_state = self.local_state.lock().await;

        prompter.show_hand(local_state.borrow());

        loop {
            let result = prompter.prompt(local_state.borrow());

            match result {
                Ok(game_event) => break game_event,
                Err(e) => say(format!("Invalid input: {}", e)),
            }
        }
    }

    async fn on_turn_start(&mut self, _game_state: &salt_engine::game_state::GameState) {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct ConsolePrompter {
    id: PlayerId,
}

impl Prompter for ConsolePrompter {
    fn prompt_slot(&self) -> BoardPos {
        let mut empty_queue = VecDeque::new();

        say("Enter the letter of any board slot.");

        loop {
            match self.prompt_pos(&mut empty_queue) {
                Ok(board_pos) => return board_pos,
                Err(e) => say(format!("{}", e)),
            }
        }
    }

    fn prompt_player_slot(&self) -> BoardPos {
        let mut empty_queue = VecDeque::new();

        say("Enter the letter of a slot you control.");

        loop {
            match self.prompt_pos(&mut empty_queue) {
                Ok(board_pos) => return board_pos,
                Err(e) => say(format!("{}", e)),
            }
        }
    }

    fn prompt_opponent_slot(&self) -> BoardPos {
        let mut empty_queue = VecDeque::new();

        say("Enter the letter of a slot your opponent controls.");

        loop {
            match self.prompt_pos(&mut empty_queue) {
                Ok(board_pos) => return board_pos,
                Err(e) => say(format!("{}", e)),
            }
        }
    }

    fn prompt_creature_pos(&self) -> BoardPos {
        let mut empty_queue = VecDeque::new();

        say("Enter the letter of any slot containing a creature.");

        loop {
            match self.prompt_pos(&mut empty_queue) {
                Ok(board_pos) => return board_pos,
                Err(e) => say(format!("{}", e)),
            }
        }
    }

    fn prompt_player_creature_pos(&self) -> BoardPos {
        let mut empty_queue = VecDeque::new();

        let game_state: &GameStatePlayerView = todo!();

        if !game_state
            .iter()
            .for_player(self.id())
            .creatures()
            .has_any()
        {
            panic!("Can't prompt for a friendly creature if there is none.");
        }

        say("Enter the letter of a slot containing a creature you control.");

        let validate = |board_pos: BoardPos| -> Result<BoardPos, ConsoleError> {
            if board_pos.player_id != self.id() {
                Err(ConsoleError::UserInputError(
                    "That position is not yours.".to_owned(),
                ))
            } else if game_state.board().creature_at_pos(board_pos).is_none() {
                Err(ConsoleError::UserInputError(
                    "That position doesn't contain a creature.".to_owned(),
                ))
            } else {
                Ok(board_pos)
            }
        };

        let friendly_creature_pos = loop {
            let any_pos = retry_until_ok(
                || self.prompt_pos(&mut empty_queue),
                |e| say(format!("{}", e)),
            );

            match validate(any_pos) {
                Ok(p) => break p,
                Err(e) => say(format!("{}", e)),
            }
        };

        friendly_creature_pos
    }

    fn prompt_opponent_creature_pos(&self) -> BoardPos {
        let mut empty_queue = VecDeque::new();

        let game_state: &GameStatePlayerView = todo!();

        say("Enter the letter of a slot containing a creature your opponent controls.");

        let validate = |board_pos: BoardPos| -> Result<BoardPos, ConsoleError> {
            if board_pos.player_id == self.id() {
                Err(ConsoleError::UserInputError(
                    "That's not an enemy position.".to_owned(),
                ))
            } else if game_state.board().creature_at_pos(board_pos).is_none() {
                Err(ConsoleError::UserInputError(
                    "That position doesn't contain a creature.".to_owned(),
                ))
            } else {
                Ok(board_pos)
            }
        };

        let enemy_creature_pos = loop {
            let any_pos = retry_until_ok(
                || self.prompt_pos(&mut empty_queue),
                |e| say(format!("{}", e)),
            );

            match validate(any_pos) {
                Ok(p) => break p,
                Err(e) => say(format!("{}", e)),
            }
        };

        enemy_creature_pos
    }
}

impl ConsolePrompter {
    fn new(id: PlayerId) -> Self {
        Self { id }
    }

    fn id(&self) -> PlayerId {
        self.id
    }

    fn prompt(&self, local_state: &LocalState) -> Result<ClientAction, ConsoleError> {
        let mut input_queue = VecDeque::new();

        let mut event = None;

        while event.is_none() {
            info!("Current state: {local_state:#?}");
            let my_hero = local_state
                .find_type::<PlayerHero>()
                .find(|h| h.player_id() == self.id())
                .unwrap();

            let mana_available = my_hero.mana_available();
            let mana_limit = my_hero.mana_limit();
            let action = self.ask(
                &format!("({}/{} mana) Enter an action: (summon, (show) board, (show) hand, info, attack, end (turn), quit)", mana_available, mana_limit),
                &mut input_queue,
            );

            event = match action.as_str() {
                "hand" => {
                    // self.show_hand(game_state);
                    self.show_hand(local_state);
                    None
                }
                "state" => {
                    // self.show_hand(game_state);
                    self.show_state(local_state);
                    None
                }
                "board" => {
                    self.show_board(local_state);
                    None
                }
                "summon" => Some(self.summon(todo!(), local_state, &mut input_queue)),
                "info" => {
                    self.info(todo!(), &mut input_queue);
                    None
                }
                "attack" => Some(self.attack(todo!(), &mut input_queue)),
                "end" => Some(Ok(ClientAction::EndTurn(EndTurn {
                    player_id: self.id(),
                }))),
                "quit" => panic!(),
                _ => None,
            };
        }

        event.unwrap()
    }

    fn summon(
        &self,
        game_state: &GameStatePlayerView,
        local_state: &LocalState,
        input_queue: &mut VecDeque<String>,
    ) -> Result<ClientAction, ConsoleError> {
        let player_id = game_state.cur_player_turn();

        let selected_card_id = {
            // self.show_hand(game_state);
            self.show_hand(local_state);

            let hand_size = game_state.hand().len();

            let card_index: usize = self
                .ask(&format!("which card? (0..={})", hand_size - 1), input_queue)
                .parse()
                .map_err(|_| ConsoleError::UserInputError("Not a valid input.".to_owned()))?;

            if card_index > game_state.hand().len() {
                return Err(user_input_err("That index is out of range."));
            }

            let selected_card = game_state
                .hand()
                .nth(card_index)
                .ok_or_else(|| user_input_err("Not a valid card index."))?;

            selected_card.id()
        };

        let board_pos = self.prompt_pos(input_queue)?;

        // let event = SummonCreatureFromHandEvent::new(player_id, board_pos, selected_card_id);
        let event = SummonCreatureFromHand {
            player_id,
            board_pos,
            card_id: selected_card_id,
        };

        Ok(ClientAction::SummonCreatureFromHand(event))

        // event
        //     .validate(game_state)
        //     .map(|_| ClientAction::SummonCreatureFromHand(event))
        //     .map_err(|e| ConsoleError::UserInputError(format!("{:?}", e)))
    }

    fn attack(
        &self,
        game_state: &GameStatePlayerView,
        input_queue: &mut VecDeque<String>,
    ) -> Result<ClientAction, ConsoleError> {
        let attacker_pos = self.prompt_pos(input_queue)?;

        if !game_state
            .iter()
            .for_player(self.id())
            .with_creature()
            .any(|s| s.pos() == attacker_pos)
        {
            return Err(ConsoleError::UserInputError(
                "That's not a valid attacker.".to_owned(),
            ));
        }

        let target_pos = self.prompt_pos(input_queue)?;

        let target_creature =
            if let Some(target_creature) = game_state.board().creature_at_pos(target_pos) {
                target_creature
            } else {
                return Err(ConsoleError::UserInputError(
                    "That's not a valid target.".to_owned(),
                ));
            };

        let attacker_id = game_state
            .board()
            .creature_at_pos(attacker_pos)
            .unwrap()
            .id();

        let target_id = target_creature.id();

        Ok(ClientAction::Attack(Attack {
            attacker: attacker_id,
            target: target_id,
        }))

        // let event = AttackEvent::new(attacker_id, target_id);

        // event
        //     .validate(game_state)
        //     .map(|_| ClientAction::Attack(event))
        //     .map_err(|e| ConsoleError::UserInputError(format!("{:?}", e)))
    }

    fn info(&self, game_state: &GameStatePlayerView, input_queue: &mut VecDeque<String>) {
        let _selected = self.select(game_state, "Select for info.", input_queue);
    }

    // fn show_board(&self, game_state: &GameStatePlayerView) {
    fn show_board(&self, game_state: &LocalState) {
        ConsoleDisplay.display(game_state);
    }

    fn show_state(&self, state: &LocalState) {
        say(format!("{:#?}", state));
    }

    fn show_hand(&self, state: &LocalState) {
        let mut result = String::new();

        let cards_in_hand = state.cards_in_player_hand(self.id);
        let cards_in_hand = cards_in_hand.collect::<Vec<_>>();
        let cards_count = cards_in_hand.len();

        info!("Found {cards_count} cards in player's hand");

        let mut cards_stringified = cards_in_hand
            .iter()
            .enumerate()
            .map(|(index, c)| display_card(c.definition(), true, index))
            .map(|s| s.lines().map(|l| l.to_owned()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        loop {
            if !cards_stringified.iter().any(|c| !c.is_empty()) {
                break;
            }

            for card_lines in &mut cards_stringified {
                result.push_str(&card_lines.remove(0));
                result.push_str("   ");
            }

            result.push('\n');
        }

        println!("{}", result);
    }

    fn select<'a>(
        &self,
        game_state: &'a GameStatePlayerView,
        ask: &str,
        input_queue: &mut VecDeque<String>,
    ) -> Option<&'a UnitCardInstancePlayerView> {
        say(ask);
        let pos = self.prompt_pos(input_queue).ok()?;
        let item_at = game_state.board().creature_at_pos(pos);

        say(format!("Selected: {:?}", item_at));

        item_at
    }

    fn prompt_pos(
        &self,
        // game_state: &GameStatePlayerView,
        input_queue: &mut VecDeque<String>,
    ) -> Result<BoardPos, ConsoleError> {
        let c = self.ask("Letter position: ", input_queue);
        let input_c = c.chars().next().ok_or_else(|| {
            ConsoleError::UserInputError("Input was not a valid character.".to_owned())
        })?;

        let game_state: &GameStatePlayerView = todo!();

        // special case for Y/Z as hero pos
        if input_c == 'Y' {
            let player_id = game_state.opponent_id();
            return Ok(BoardPos::hero_pos(player_id));
        } else if input_c == 'Z' {
            let player_id = game_state.player_id();
            return Ok(BoardPos::hero_pos(player_id));
        }

        let enemy_back_chars = "ABCDEF".chars();
        let enemy_front_chars = "GHIJKL".chars();
        let my_front_chars = "MNOPQR".chars();
        let my_back_chars = "STUVWX".chars();

        let board_pos = if let Some((index, _)) =
            enemy_back_chars.enumerate().find(|&(_, c)| c == input_c)
        {
            BoardPos::new(game_state.opponent_id(), RowId::BackRow, index)
        } else if let Some((index, _)) = enemy_front_chars.enumerate().find(|&(_, c)| c == input_c)
        {
            BoardPos::new(game_state.opponent_id(), RowId::FrontRow, index)
        } else if let Some((index, _)) = my_front_chars.enumerate().find(|&(_, c)| c == input_c) {
            BoardPos::new(game_state.player_id(), RowId::FrontRow, index)
        } else if let Some((index, _)) = my_back_chars.enumerate().find(|&(_, c)| c == input_c) {
            BoardPos::new(game_state.player_id(), RowId::BackRow, index)
        } else {
            return Err(ConsoleError::UserInputError(format!(
                "The input char {} did not match any position",
                input_c
            )));
        };

        Ok(board_pos)
    }

    fn ask(&self, message: &str, input_queue: &mut VecDeque<String>) -> String {
        if let Some(input) = input_queue.pop_front() {
            return input;
        }

        say(message);

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("stdin readline failed");

        for token in input.split_whitespace() {
            input_queue.push_back(token.into());
        }

        input_queue.pop_front().expect("No input provided.")
    }
}

fn say(message: impl AsRef<str>) {
    println!("{}", message.as_ref());
}

fn display_card(card: &CreatureDefinition, playable: bool, tag: usize) -> String {
    let text_lines = card.text.lines().collect::<Vec<_>>();

    const WIDTH: usize = 26;

    let border = match playable {
        true => '+',
        false => '-',
    };

    let border = std::iter::repeat(border).take(WIDTH).collect::<String>();

    format!(
        r#"{}
|{:<21} {} |
|{}|
|{:^24}|
|{:^24}|
|{:^24}|
|{:^24}|
|{:^24}|
|{:^24}|
|{:^24}|
|{:^24}|
| W: {}              {}/{}  |
{}
{:^26}"#,
        border,
        card.title,
        card.cost,
        &border[..border.len() - 2],
        text_lines.get(0).unwrap_or(&""),
        text_lines.get(1).unwrap_or(&""),
        text_lines.get(2).unwrap_or(&""),
        text_lines.get(3).unwrap_or(&""),
        text_lines.get(4).unwrap_or(&""),
        text_lines.get(5).unwrap_or(&""),
        text_lines.get(6).unwrap_or(&""),
        text_lines.get(7).unwrap_or(&""),
        card.row_width,
        card.attack,
        card.health,
        border,
        tag
    )
}

fn retry_until_ok<TOut, TErr>(
    mut action: impl FnMut() -> Result<TOut, TErr>,
    mut on_err: impl FnMut(TErr),
) -> TOut {
    loop {
        match (action)() {
            Ok(ok) => return ok,
            Err(e) => (on_err)(e),
        }
    }
}

struct ConsoleNotifier {
    sender: Sender<Notification>,
    local_state: Arc<Mutex<LocalState>>,
}

impl ConsoleNotifier {
    fn new(local_state: Arc<Mutex<LocalState>>) -> (Self, Receiver<Notification>) {
        let (sender, receiver) = smol::channel::unbounded::<Notification>();
        (
            Self {
                sender,
                local_state,
            },
            receiver,
        )
    }

    async fn send(&self, notification: Notification) -> Result<(), SendError<Notification>> {
        self.sender.send(notification).await
    }

    fn update_entity(entity_update: EntityUpdate, local_state: &mut LocalState) {
        let mut entity = local_state.find_entity(entity_update.id);
    }

    fn add_entity(entity_added: EntityAdded, local_state: &mut LocalState) {
        local_state.add_at(entity_added.entity, entity_added.position);
    }
}

#[async_trait]
impl ClientNotifier for ConsoleNotifier {
    async fn notify(&self, event: Notification) {
        info!("Saw client event: {event:?}");

        info!("Locking local state...");
        let mut guard = self.local_state.lock().await;
        let local_state = guard.borrow_mut();
        info!("Locked local state.");

        match event {
            Notification::VisualEvent(e) => info!("Saw visual event: {e:?}"),
            Notification::EntityUpdate(e) => ConsoleNotifier::update_entity(e, local_state),
            Notification::EntityAdded(e) => ConsoleNotifier::add_entity(e, local_state),
        }
    }
}
