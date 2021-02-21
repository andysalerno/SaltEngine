use std::{collections::VecDeque, io::stdin};

use super::game_agent::{GameAgent, Prompter};
use crate::game_state::board::RowId;
use crate::{game_logic::Event, game_state::board::BoardPos};
use crate::{
    game_logic::{cards::UnitCardDefinition, GameEvent, SummonCreatureFromHandEvent},
    game_state::PlayerId,
};
use crate::{
    game_logic::{AttackEvent, EndTurnEvent},
    game_state::{GameState, UnitCardInstance},
};

use std::error::Error;

pub struct ConsoleAgent {
    id: PlayerId,
}

impl ConsoleAgent {
    pub fn new() -> Self {
        Self {
            id: PlayerId::new(),
        }
    }
}

impl GameAgent for ConsoleAgent {
    fn get_action(&self, game_state: &GameState) -> GameEvent {
        let prompter = ConsolePrompter::new(self.id());
        prompter.show_hand(game_state);

        loop {
            let result = prompter.prompt(game_state);

            match result {
                Ok(game_event) => break game_event,
                Err(e) => say(&format!("Invalid input: {}", e.to_string())),
            }
        }
    }

    fn id(&self) -> PlayerId {
        self.id
    }

    fn make_prompter(&self) -> Box<dyn Prompter> {
        Box::new(ConsolePrompter::new(self.id()))
    }
}

#[derive(Debug, Clone)]
struct ConsolePrompter {
    id: PlayerId,
}

impl Prompter for ConsolePrompter {
    fn prompt_slot(&self, game_state: &GameState) -> BoardPos {
        let mut empty_queue = VecDeque::new();
        self.prompt_pos_any(game_state, &mut empty_queue)
    }

    fn prompt_player_slot(&self, _game_state: &GameState) -> BoardPos {
        todo!()
    }

    fn prompt_opponent_slot(&self, _game_state: &GameState) -> BoardPos {
        todo!()
    }

    fn prompt_creature_pos(&self, _game_state: &GameState) -> BoardPos {
        todo!()
    }

    fn prompt_player_creature_pos(&self, _game_state: &GameState) -> BoardPos {
        todo!()
    }

    fn prompt_opponent_creature_pos(&self, _game_state: &GameState) -> BoardPos {
        todo!()
    }
}

impl ConsolePrompter {
    fn new(id: PlayerId) -> Self {
        Self { id }
    }

    fn id(&self) -> PlayerId {
        self.id
    }

    fn prompt(&self, game_state: &GameState) -> Result<GameEvent, Box<dyn Error>> {
        let mut input_queue = VecDeque::new();

        let mut event = None;

        while event.is_none() {
            let action = self.ask(
                "Enter an action: (summon, (show) board, (show) hand, info, attack, end (turn), quit)",
                &mut input_queue,
            );

            event = match action.as_str() {
                "hand" => {
                    self.show_hand(game_state);
                    None
                }
                "board" => {
                    self.show_hand(game_state);
                    None
                }
                "summon" => Some(self.summon(game_state, &mut input_queue)),
                "info" => {
                    self.info(game_state, &mut input_queue);
                    None
                }
                "attack" => Some(self.attack(game_state, &mut input_queue)),
                "end" => Some(Ok(EndTurnEvent.into())),
                "quit" => panic!(),
                _ => panic!("Unknown input: {}", action),
            };
        }

        event.unwrap()
    }

    fn summon(
        &self,
        game_state: &GameState,
        input_queue: &mut VecDeque<String>,
    ) -> Result<GameEvent, Box<dyn Error>> {
        let player_id = game_state.cur_player_id();

        let selected_card_id = {
            self.show_hand(game_state);

            let hand_size = game_state.hand(player_id).len();

            let card_index: usize = self
                .ask(&format!("which card? (0..={})", hand_size - 1), input_queue)
                .parse()
                .expect("invalid input");

            let selected_card = game_state.hand(player_id).nth(card_index);

            selected_card.id()
        };

        let board_pos = self.prompt_pos_any(game_state, input_queue);

        let event = SummonCreatureFromHandEvent::new(player_id, board_pos, selected_card_id);

        event.validate(game_state).map(|_| event.into())
    }

    fn attack(
        &self,
        game_state: &GameState,
        input_queue: &mut VecDeque<String>,
    ) -> Result<GameEvent, Box<dyn Error>> {
        let attacker = loop {
            let pos = self.prompt_pos_myside(game_state, input_queue);
            match game_state.creature_at_pos(pos) {
                Some(c) => break c.id(),
                _ => say("No card found at that pos; try again"),
            }
        };

        let target = loop {
            let pos = self.prompt_pos_enemyside(game_state, input_queue);
            match game_state.creature_at_pos(pos) {
                Some(c) => break c.id(),
                _ => say("No card found at that pos; try again"),
            }
        };

        let event = AttackEvent::new(attacker, target);

        event.validate(game_state).map(|_| event.into())
    }

    fn info(&self, game_state: &GameState, input_queue: &mut VecDeque<String>) {
        let _selected = self.select(game_state, "Select for info.", input_queue);
    }

    fn show_hand(&self, game_state: &GameState) {
        let mut result = String::new();

        let mut all_cards = game_state
            .hand(self.id())
            .cards()
            .iter()
            .enumerate()
            .map(|(index, c)| display_card(c.definition(), index))
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

            result.push_str("\n");
        }

        println!("{}", result);
    }

    fn select<'a>(
        &self,
        game_state: &'a GameState,
        ask: &str,
        input_queue: &mut VecDeque<String>,
    ) -> Option<&'a UnitCardInstance> {
        say(ask);
        let pos = self.prompt_pos_any(game_state, input_queue);
        let item_at = game_state.creature_at_pos(pos);

        say(&format!("Selected: {:?}", item_at));

        item_at
    }

    fn prompt_pos_any(
        &self,
        game_state: &GameState,
        input_queue: &mut VecDeque<String>,
    ) -> BoardPos {
        let c = self.ask("Select any position on the board.", input_queue);
        let input_c = c.chars().nth(0).expect("Expected single-char response");

        let enemy_back_chars = "ABCDEF".chars();
        let enemy_front_chars = "GHIJKL".chars();
        let my_front_chars = "MNOPQR".chars();
        let my_back_chars = "STUVWX".chars();

        if let Some((index, _)) = enemy_back_chars.enumerate().find(|&(_, c)| c == input_c) {
            return BoardPos::new(game_state.player_b_id(), RowId::BackRow, index);
        } else if let Some((index, _)) = enemy_front_chars.enumerate().find(|&(_, c)| c == input_c)
        {
            return BoardPos::new(game_state.player_b_id(), RowId::FrontRow, index);
        } else if let Some((index, _)) = my_front_chars.enumerate().find(|&(_, c)| c == input_c) {
            return BoardPos::new(game_state.player_a_id(), RowId::FrontRow, index);
        } else if let Some((index, _)) = my_back_chars.enumerate().find(|&(_, c)| c == input_c) {
            return BoardPos::new(game_state.player_a_id(), RowId::BackRow, index);
        }

        panic!("The input char {} did not match any position", input_c);
    }

    fn prompt_pos_myside(
        &self,
        _game_state: &GameState,
        input_queue: &mut VecDeque<String>,
    ) -> BoardPos {
        let player = self.id();
        let row = self.prompt_row(input_queue);
        let index = self.prompt_row_index(input_queue);

        BoardPos::new(player, row, index)
    }

    fn prompt_pos_enemyside(
        &self,
        game_state: &GameState,
        input_queue: &mut VecDeque<String>,
    ) -> BoardPos {
        let player = game_state.other_player(self.id());
        let row = self.prompt_row(input_queue);
        let index = self.prompt_row_index(input_queue);

        BoardPos::new(player, row, index)
    }

    fn prompt_player(
        &self,
        game_state: &GameState,
        input_queue: &mut VecDeque<String>,
    ) -> PlayerId {
        let player_in = self.ask("Which player? (me, opponent)", input_queue);

        match player_in.as_str() {
            "me" => self.id(),
            "opponent" => game_state.other_player(self.id()),
            _ => panic!("Unknown input: {}", player_in),
        }
    }

    fn prompt_row(&self, input_queue: &mut VecDeque<String>) -> RowId {
        let row_in = self.ask("Which row? (front, back)", input_queue);

        match row_in.as_str() {
            "front" => RowId::FrontRow,
            "back" => RowId::BackRow,
            _ => panic!("Unknown input: {}", row_in),
        }
    }

    fn prompt_row_index(&self, input_queue: &mut VecDeque<String>) -> usize {
        let row_index = self.ask("What row index? (0..=5)", input_queue);

        let index = row_index.parse::<usize>().expect("Invalid index");

        if !(0..6).contains(&index) {
            panic!("not in range");
        }

        index
    }

    fn ask(&self, message: &str, input_queue: &mut VecDeque<String>) -> String {
        if let Some(input) = input_queue.pop_front() {
            return input;
        }

        say(message);

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("stdin readline failed");

        for token in input.split_whitespace() {
            input_queue.push_back(token.into());
        }

        input_queue.pop_front().expect("No input provided.")
    }
}

fn say(message: &str) {
    println!("{}", message);
}

fn display_card(card: &dyn UnitCardDefinition, tag: usize) -> String {
    let text_lines = card.text().lines().collect::<Vec<_>>();

    format!(
        r#"-----------------------
|{:<18} {} |
|---------------------|
|{:^21}|
|{:^21}|
|{:^21}|
|{:^21}|
|{:^21}|
|                {}/{}  |
-----------------------
{:^23}"#,
        card.title(),
        card.cost(),
        text_lines.get(0).unwrap_or(&""),
        text_lines.get(1).unwrap_or(&""),
        text_lines.get(2).unwrap_or(&""),
        text_lines.get(3).unwrap_or(&""),
        text_lines.get(4).unwrap_or(&""),
        card.attack(),
        card.health(),
        tag
    )
}
