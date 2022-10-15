use crate::{
    event::{EventHandler, EventMessage, EventType},
    game_client::{FromServer, MessageChannel},
    FromClient, GameState, PlayerId,
};
use log::info;
use std::collections::HashMap;

pub struct ClientChannel {
    player_id: PlayerId,
    channel: Box<dyn MessageChannel<Send = FromServer, Receive = FromClient>>,
}

impl ClientChannel {
    #[must_use]
    pub fn new(
        player_id: PlayerId,
        channel: Box<dyn MessageChannel<Send = FromServer, Receive = FromClient>>,
    ) -> Self {
        Self { player_id, channel }
    }

    #[must_use]
    pub fn try_receive(&self) -> Option<FromClient> {
        self.channel.try_receive()
    }

    pub fn send(&self, message: FromServer) {
        self.channel.send(message);
    }
}

pub struct Dispatcher {
    // event_stack: ...
    event_handler_mapping: HashMap<EventType, Box<dyn EventHandler>>,
    player_a_channel: ClientChannel,
    player_b_channel: ClientChannel,
}

impl Dispatcher {
    /// # Panics
    /// Panics if there is a problem registering handlers for events.
    #[must_use]
    pub fn new(
        handlers: Vec<Box<dyn EventHandler>>,
        player_a_channel: ClientChannel,
        player_b_channel: ClientChannel,
    ) -> Self {
        // consume the handlers and map them.
        let handlers_provided = handlers.len();

        let mut mapping = HashMap::new();

        for handler in handlers {
            let event_type = handler.event_type();
            info!("Registered handler for event type {event_type:?}");
            mapping.insert(event_type, handler);
        }

        assert!(
            handlers_provided == mapping.keys().len(),
            "Conflict in event handlers and handled types."
        );

        Self {
            event_handler_mapping: mapping,
            player_a_channel,
            player_b_channel,
        }
    }

    /// # Panics
    /// Panics if there is no valid handler for the event.
    pub fn dispatch(&self, event: &EventMessage, game_state: &mut GameState) {
        let event_type = event.event_type();

        let matching_handler = self
            .event_handler_mapping
            .get(event_type)
            .unwrap_or_else(|| {
                panic!("no matching handler was found for event type {event_type:?}")
            });

        info!("Dispatching event {event:?}");

        self.player_a_channel
            .channel
            .send(FromServer::Event(event.clone()));
        self.player_b_channel
            .channel
            .send(FromServer::Event(event.clone()));

        matching_handler.handle(event, game_state, self);
    }

    #[must_use]
    pub const fn player_a_channel(&self) -> &ClientChannel {
        &self.player_a_channel
    }

    /// Provides the `ClientChannel` for the given player.
    ///
    /// # Panics
    ///
    /// Panics if a value is provided for `player_id` that does not match either player.
    #[must_use]
    pub fn player_channel(&self, player_id: PlayerId) -> &ClientChannel {
        if player_id == self.player_a_channel.player_id {
            &self.player_a_channel
        } else if player_id == self.player_b_channel.player_id {
            &self.player_b_channel
        } else {
            panic!("Unknown player ID provided: {player_id:?}");
        }
    }

    #[must_use]
    pub const fn player_b_channel(&self) -> &ClientChannel {
        &self.player_b_channel
    }
}
