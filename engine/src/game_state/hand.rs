use super::{
    card_instance::{UnitCardInstancePlayerView, UnitCardInstanceView},
    MakePlayerView, PlayerId, UnitCardInstance,
};
use protocol::entities::UnitCardInstanceId;
use serde::{Deserialize, Serialize};

pub trait HandView<'a> {
    type TCard: UnitCardInstanceView<'a>;

    fn cards(&self) -> &[Self::TCard];

    fn card(&self, id: UnitCardInstanceId) -> &Self::TCard {
        self.cards()
            .iter()
            .find(|c| c.id() == id)
            .unwrap_or_else(|| {
                panic!(
                    "Attempted to find card with id {:?} in hand, but no such card was found.",
                    id
                )
            })
    }

    fn nth(&self, n: usize) -> Option<&Self::TCard> {
        self.cards().get(n)
    }

    fn len(&self) -> usize {
        self.cards().len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Default)]
pub struct Hand {
    cards: Vec<UnitCardInstance>,
}

impl Hand {
    #[must_use]
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn cards(&self) -> &[UnitCardInstance] {
        self.cards.as_slice()
    }

    pub fn cards_mut(&mut self) -> &mut [UnitCardInstance] {
        self.cards.as_mut_slice()
    }

    pub fn add_card(&mut self, card: UnitCardInstance) {
        self.cards.push(card);
    }

    #[must_use]
    pub fn card(&self, id: UnitCardInstanceId) -> &UnitCardInstance {
        HandView::card(self, id)
    }

    pub fn take_card(&mut self, id: UnitCardInstanceId) -> UnitCardInstance {
        let (index, _) = self
            .cards
            .iter()
            .enumerate()
            .find(|(_i, c)| c.id() == id)
            .unwrap_or_else(|| {
                panic!(
                    "Attempted to take card with id {:?} from hand, but no such card was found.",
                    id
                )
            });

        let card = self.cards.remove(index);
        assert!(card.id() == id);

        card
    }
}

impl<'a> HandView<'a> for Hand {
    type TCard = UnitCardInstance;

    fn cards(&self) -> &[UnitCardInstance] {
        Hand::cards(self)
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct HandPlayerView {
    cards: Vec<UnitCardInstancePlayerView>,
}

impl<'a> MakePlayerView<'a> for Hand {
    type TOut = HandPlayerView;

    fn player_view(&'a self, player_viewing: PlayerId) -> HandPlayerView {
        let cards = self
            .cards()
            .iter()
            .map(|c| c.player_view(player_viewing))
            .collect();

        HandPlayerView { cards }
    }
}

impl<'a> HandView<'a> for HandPlayerView {
    type TCard = UnitCardInstancePlayerView;

    fn cards(&self) -> &[UnitCardInstancePlayerView] {
        self.cards.as_slice()
    }
}
