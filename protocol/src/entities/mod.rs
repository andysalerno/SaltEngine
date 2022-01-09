mod board;
mod buff;
mod hideable;
mod id;
mod passive_effect;
mod unit_card_definition;
mod unit_card_instance_view;

pub use board::*;
pub use buff::*;
pub use hideable::*;
pub use id::*;
pub use passive_effect::*;
use serde::{Deserialize, Serialize};
pub use unit_card_instance_view::*;

trait Entity<'a>: Serialize + Deserialize<'a> {
    fn entity_id(&self) -> Id;
}
