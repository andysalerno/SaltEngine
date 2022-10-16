use engine::CardDefinition;

pub struct SleepingDog;

impl SleepingDog {
    pub fn make_definition() -> CardDefinition {
        CardDefinition::builder()
            .title("Sleeping Dog")
            .attack(0)
            .health(3)
            .cost(1)
            .build()
    }
}
