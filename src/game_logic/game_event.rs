pub trait GameEvent {}

pub struct AttackEvent;
pub struct EndTurnEvent;

impl GameEvent for AttackEvent {}
impl GameEvent for EndTurnEvent {}
