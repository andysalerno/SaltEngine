mod draw_card_event;

pub use draw_card_event::{DrawCardEvent, DrawCardEventHandler};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
