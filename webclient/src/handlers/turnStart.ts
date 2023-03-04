import { getContext } from "../js";
import { PlayerStartTurnEvent } from "../message";
import { activateEndTurnBox, logGameMessage } from "../util";

export function handleTurnStart(event: PlayerStartTurnEvent) {
    if (event.player_id.guid === getContext().myId) {
        myTurnStart(event);
    } else {
        enemyTurnStart(event);
    }
}

function myTurnStart(event: PlayerStartTurnEvent) {
    logGameMessage("My turn started. Mana: " + event.starting_mana);
    const context = getContext();
    context.myMana = event.starting_mana;
    context.isMyTurn = true;

    activateHand();
    activateEndTurnBox();
}

function enemyTurnStart(event: PlayerStartTurnEvent) {
    logGameMessage("Enemy turn started. Mana: " + event.starting_mana);
}

function activateHand() {
    const context = getContext();
    for (let handCard of context.myHand.cards) {
        if (handCard.current_cost <= context.myMana) {
            // The player can play this card, so add a visual queue.

        }
    }
}