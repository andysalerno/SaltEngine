use crate::console_display::ConsoleDisplay;
use async_trait::async_trait;
use protocol::{
    client_actions::{Attack, EndTurn, SummonCreatureFromHand},
    entities::{BoardPos, PlayerId, RowId},
    ClientAction, VisualEvent,
};
use salt_engine::{
    cards::{player_view::UnitCardDefinitionPlayerView, UnitCardDefinitionView},
    game_agent::{ClientNotifier, Prompter},
    game_logic::events::{AttackEvent, Event},
    game_runner::GameClient,
    game_state::{
        board::BoardView, GameStatePlayerView, GameStateView, HandView, IterAddons, IteratorAny,
        UnitCardInstancePlayerView,
    },
};
use std::collections::VecDeque;
use thiserror::Error;

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
}

impl ConsoleAgent {
    pub fn new() -> Self {
        Self::new_with_id(PlayerId::new())
    }

    pub fn new_with_id(id: PlayerId) -> Self {
        Self { id }
    }

    fn id(&self) -> PlayerId {
        self.id
    }
}

#[async_trait]
impl GameClient for ConsoleAgent {
    async fn make_prompter(&self) -> Box<dyn Prompter> {
        Box::new(ConsolePrompter::new(self.id()))
    }

    async fn make_notifier(&self) -> Box<dyn ClientNotifier> {
        Box::new(ConsoleNotifier)
    }

    async fn next_action(&mut self, game_state: GameStatePlayerView) -> ClientAction {
        let prompter = ConsolePrompter::new(self.id());
        prompter.show_hand(&game_state);

        loop {
            let result = prompter.prompt(&game_state);

            match result {
                Ok(game_event) => break game_event,
                Err(e) => say(format!("Invalid input: {}", e.to_string())),
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
    fn prompt_slot(&self, game_state: &GameStatePlayerView) -> BoardPos {
        let mut empty_queue = VecDeque::new();

        say("Enter the letter of any board slot.");

        loop {
            match self.prompt_pos(game_state, &mut empty_queue) {
                Ok(board_pos) => return board_pos,
                Err(e) => say(format!("{}", e)),
            }
        }
    }

    fn prompt_player_slot(&self, game_state: &GameStatePlayerView) -> BoardPos {
        let mut empty_queue = VecDeque::new();

        say("Enter the letter of a slot you control.");

        loop {
            match self.prompt_pos(game_state, &mut empty_queue) {
                Ok(board_pos) => return board_pos,
                Err(e) => say(format!("{}", e)),
            }
        }
    }

    fn prompt_opponent_slot(&self, game_state: &GameStatePlayerView) -> BoardPos {
        let mut empty_queue = VecDeque::new();

        say("Enter the letter of a slot your opponent controls.");

        loop {
            match self.prompt_pos(game_state, &mut empty_queue) {
                Ok(board_pos) => return board_pos,
                Err(e) => say(format!("{}", e)),
            }
        }
    }

    fn prompt_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos {
        let mut empty_queue = VecDeque::new();

        say("Enter the letter of any slot containing a creature.");

        loop {
            match self.prompt_pos(game_state, &mut empty_queue) {
                Ok(board_pos) => return board_pos,
                Err(e) => say(format!("{}", e)),
            }
        }
    }

    fn prompt_player_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos {
        let mut empty_queue = VecDeque::new();

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
                || self.prompt_pos(game_state, &mut empty_queue),
                |e| say(format!("{}", e)),
            );

            match validate(any_pos) {
                Ok(p) => break p,
                Err(e) => say(format!("{}", e)),
            }
        };

        friendly_creature_pos
    }

    fn prompt_opponent_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos {
        let mut empty_queue = VecDeque::new();

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
                || self.prompt_pos(game_state, &mut empty_queue),
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

    fn prompt(&self, game_state: &GameStatePlayerView) -> Result<ClientAction, ConsoleError> {
        let mut input_queue = VecDeque::new();

        let mut event = None;

        while event.is_none() {
            let available_mana = game_state.player_mana(self.id());
            let mana_limit = game_state.player_mana_limit(self.id());
            let action = self.ask(
                &format!("({}/{} mana) Enter an action: (summon, (show) board, (show) hand, info, attack, end (turn), quit)", available_mana, mana_limit),
                &mut input_queue,
            );

            event = match action.as_str() {
                "hand" => {
                    self.show_hand(game_state);
                    None
                }
                "board" => {
                    self.show_board(game_state);
                    None
                }
                "summon" => Some(self.summon(game_state, &mut input_queue)),
                "info" => {
                    self.info(game_state, &mut input_queue);
                    None
                }
                "attack" => Some(self.attack(game_state, &mut input_queue)),
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
        input_queue: &mut VecDeque<String>,
    ) -> Result<ClientAction, ConsoleError> {
        let player_id = game_state.cur_player_turn();

        let selected_card_id = {
            self.show_hand(game_state);

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

        let board_pos = self.prompt_pos(game_state, input_queue)?;

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
        let attacker_pos = self.prompt_pos(game_state, input_queue)?;

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

        let target_pos = self.prompt_pos(game_state, input_queue)?;

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

    fn show_board(&self, game_state: &GameStatePlayerView) {
        ConsoleDisplay.display(game_state);
    }

    fn show_hand(&self, game_state: &GameStatePlayerView) {
        let mut result = String::new();

        let available_mana = game_state.player_mana(self.id());

        let mut all_cards = game_state
            .hand()
            .cards()
            .iter()
            .enumerate()
            .map(|(index, c)| {
                display_card(
                    c.definition(),
                    c.definition().cost() <= available_mana as i32,
                    index,
                )
            })
            .map(|s| s.lines().map(|l| l.to_owned()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        loop {
            if !all_cards.iter().any(|c| !c.is_empty()) {
                break;
            }

            for card_lines in &mut all_cards {
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
        let pos = self.prompt_pos(game_state, input_queue).ok()?;
        let item_at = game_state.board().creature_at_pos(pos);

        say(format!("Selected: {:?}", item_at));

        item_at
    }

    fn prompt_pos(
        &self,
        game_state: &GameStatePlayerView,
        input_queue: &mut VecDeque<String>,
    ) -> Result<BoardPos, ConsoleError> {
        let c = self.ask("Letter position: ", input_queue);
        let input_c = c.chars().next().ok_or_else(|| {
            ConsoleError::UserInputError("Input was not a valid character.".to_owned())
        })?;

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

// fn display_card(card: &dyn UnitCardDefinition, playable: bool, tag: usize) -> String {
fn display_card(card: &UnitCardDefinitionPlayerView, playable: bool, tag: usize) -> String {
    let text_lines = card.text().lines().collect::<Vec<_>>();

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
        card.title(),
        card.cost(),
        &border[..border.len() - 2],
        text_lines.get(0).unwrap_or(&""),
        text_lines.get(1).unwrap_or(&""),
        text_lines.get(2).unwrap_or(&""),
        text_lines.get(3).unwrap_or(&""),
        text_lines.get(4).unwrap_or(&""),
        text_lines.get(5).unwrap_or(&""),
        text_lines.get(6).unwrap_or(&""),
        text_lines.get(7).unwrap_or(&""),
        card.row_width(),
        card.attack(),
        card.health(),
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

struct ConsoleNotifier;

#[async_trait]
impl ClientNotifier for ConsoleNotifier {
    async fn notify(&self, event: VisualEvent) {
        say(format!("Saw client event: {:?}", event));
    }
}
