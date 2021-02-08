use std::io::stdin;

use super::game_agent::GameAgent;
use crate::game_logic::{cards::UnitCardDefinition, GameEvent};
use crate::game_state::board::BoardPos;
use crate::{
    game_logic::{AttackEvent, EndTurnEvent},
    game_state::{GameState, UnitCardInstance},
};
use crate::{game_state::board::RowId, id::Id};

pub struct ConsoleAgent {
    id: Id,
}

impl ConsoleAgent {
    pub fn new() -> Self {
        Self { id: Id::new() }
    }
}

impl GameAgent for ConsoleAgent {
    fn get_action(&self, game_state: &GameState) -> GameEvent {
        self.prompt(game_state).expect("No event selected")
    }

    fn id(&self) -> Id {
        self.id
    }
}

impl ConsoleAgent {
    fn prompt(&self, game_state: &GameState) -> Option<GameEvent> {
        let mut event = None;

        while event.is_none() {
            let action = self.ask("Enter an action: (hand, info, attack, end (turn), quit)");
            println!("Saw action: {}", action);

            event = match action.as_str() {
                "hand" => {
                    self.show_hand(game_state);
                    None
                }
                "summon" => None,
                "info" => {
                    self.info(game_state);
                    None
                }
                "attack" => Some(GameEvent::Attack(self.attack(game_state))),
                "end" => Some(GameEvent::EndTurn(EndTurnEvent)),
                "quit" => return None,
                _ => panic!("Unknown input: {}", action),
            };
        }

        event
    }

    fn summon(&self, game_state: &GameState) {
        let player_id = game_state.cur_player_id();
        self.show_hand(game_state);
        let hand_size = game_state.hand(player_id).len();
        let card_index: usize = self
            .ask(&format!("which card? (0..={})", hand_size - 1))
            .parse()
            .expect("invalid input");

        let selected_card = game_state
            .hand(player_id)
            .cards()
            .into_iter()
            .nth(card_index);

        if let Some(selected_card) = selected_card {}
    }

    fn attack(&self, game_state: &GameState) -> AttackEvent {
        let selected = self
            .select(game_state, "Select attacker.")
            .expect("Selected attacker was None.");
        let target = self
            .select(game_state, "Select target.")
            .expect("Selected target was none.");

        AttackEvent::new(selected.id(), target.id())
    }

    fn info(&self, game_state: &GameState) {
        let _selected = self.select(game_state, "Select for info.");
    }

    fn show_hand(&self, game_state: &GameState) {
        let mut result = String::new();

        let mut all_cards = game_state
            .hand(self.id())
            .cards()
            .iter()
            .map(|c| display_card(c.definition()))
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

    fn select<'a>(&self, game_state: &'a GameState, ask: &str) -> Option<&'a UnitCardInstance> {
        self.say(ask);
        let pos = self.prompt_pos(game_state);
        let item_at = game_state.get_at(pos);

        self.say(&format!("Selected: {:?}", item_at));

        item_at
    }

    fn prompt_pos(&self, game_state: &GameState) -> BoardPos {
        let player = self.prompt_player(game_state);
        let row = self.prompt_row();
        let index = self.prompt_row_index();

        BoardPos::new(player, row, index)
    }

    fn prompt_player(&self, game_state: &GameState) -> Id {
        let player_in = self.ask("Which player? (me, opponent)");

        match player_in.as_str() {
            "me" => self.id(),
            "opponent" => self.opponent_id(game_state),
            _ => panic!("Unknown input: {}", player_in),
        }
    }

    fn prompt_row(&self) -> RowId {
        let row_in = self.ask("Which row? (front, back)");

        match row_in.as_str() {
            "front" => RowId::FrontRow,
            "back" => RowId::BackRow,
            _ => panic!("Unknown input: {}", row_in),
        }
    }

    fn prompt_row_index(&self) -> usize {
        let row_index = self.ask("What row index? (0..=5)");

        let index = row_index.parse::<usize>().expect("Invalid index");

        if !(0..6).contains(&index) {
            panic!("not in range");
        }

        index
    }

    fn say(&self, message: &str) {
        println!("{}", message);
    }

    fn ask(&self, message: &str) -> String {
        self.say(message);

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("stdin readline failed");

        input.truncate(input.len() - 1);
        input
    }

    fn opponent_id(&self, game_state: &GameState) -> Id {
        if self.id() == game_state.player_a_id() {
            game_state.player_b_id()
        } else {
            game_state.player_a_id()
        }
    }
}

fn display_card(card: &dyn UnitCardDefinition) -> String {
    format!(
        r#"-----------------------
|{:<18} {} |
|---------------------|
|                     |
|                     |
|                     |
|                     |
|                     |
|                {}/{}  |
-----------------------"#,
        card.title(),
        card.cost(),
        card.attack(),
        card.health()
    )
}
