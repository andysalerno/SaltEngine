import { getContext } from "../js";
import { CardDrawnEvent } from "../message";
import { addCardToHand, logGameMessage } from "../util";

export function handleCardDrawn(event: CardDrawnEvent) {
    if (event.player_id.guid == getContext().myId) {
        logGameMessage("I drew a card.");
        addCardToHand(event.card_drawn.Visible);
    } else {
        logGameMessage("Enemy drew a card.");
    }
}