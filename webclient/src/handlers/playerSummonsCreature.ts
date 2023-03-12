import { getContext } from "../js";
import { CardOnBoard, PlayerSummonsCreatureClientEvent } from "../message";
import { logGameMessage, removeCardFromHand, setCardOnEnemyBoardSlot } from "../util";


export function handlePlayerSummonsCreature(event: PlayerSummonsCreatureClientEvent) {
    if (event.player_id.guid == getContext().enemyId) {
        logGameMessage(`Enemy summoned: ${event.definition.title}`);

        const definition = event.definition;

        let cardOnBoard: CardOnBoard = {
            title: definition.title,
            current_attack: definition.attack,
            current_cost: definition.cost,
            current_health: definition.health,
            definition: definition,
            id: event.card_id,
            can_attack: false,
        };

        setCardOnEnemyBoardSlot(cardOnBoard, event.target_pos.SlotIndex);
    } else {
        // We immediately placed it on the board when we released the moue button.
        // But now we can remove it from the hand.
        removeCardFromHand(event.card_id);
    }
}