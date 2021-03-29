use super::{
    card_instance::{UnitCardInstancePlayerView, UnitCardInstanceView},
    MakePlayerView, PlayerId, UnitCardInstance, UnitCardInstanceId,
};
use serde::{Deserialize, Serialize};

pub trait HandView<'a> {
    type TCard: UnitCardInstanceView<'a>;

    fn cards(&self) -> &[Self::TCard];

    fn card(&self, id: UnitCardInstanceId) -> &Self::TCard {
        self.cards()
            .iter()
            .filter(|c| c.id() == id)
            .next()
            .expect(&format!(
                "Attempted to find card with id {:?} in hand, but no such card was found.",
                id
            ))
    }

    fn nth(&self, n: usize) -> &Self::TCard {
        self.cards()
            .iter()
            .nth(n)
            .expect(&format!("No card at index {}", n))
    }

    fn len(&self) -> usize {
        self.cards().len()
    }
}

#[derive(Debug, Default)]
pub struct Hand {
    cards: Vec<UnitCardInstance>,
}

impl Hand {
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn cards(&self) -> &[UnitCardInstance] {
        self.cards.as_slice()
    }

    pub fn cards_mut(&mut self) -> &mut [UnitCardInstance] {
        self.cards.as_mut_slice()
    }

    pub fn add_card(&mut self, card: UnitCardInstance) {
        self.cards.push(card);
    }

    pub fn card(&self, id: UnitCardInstanceId) -> &UnitCardInstance {
        HandView::card(self, id)
    }

    pub fn take_card(&mut self, id: UnitCardInstanceId) -> UnitCardInstance {
        let (index, _) = self
            .cards
            .iter()
            .enumerate()
            .filter(|(_i, c)| c.id() == id)
            .next()
            .expect(&format!(
                "Attempted to take card with id {:?} from hand, but no such card was found.",
                id
            ));

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

#[derive(Debug, Serialize, Deserialize)]
pub struct HandPlayerView {
    cards: Vec<UnitCardInstancePlayerView>,
}

impl MakePlayerView for Hand {
    type TOut = HandPlayerView;

    fn player_view(&self, player_viewing: PlayerId) -> HandPlayerView {
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
