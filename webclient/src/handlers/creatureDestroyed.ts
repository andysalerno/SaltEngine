import { CreatureDestroyedEvent } from "../message";
import { findBoardCardById, logGameMessage, removeCardFromBoard } from "../util";

export function handleCreatureDestroyed(event: CreatureDestroyedEvent) {
    removeCardFromBoard(event.creature_destroyed);
}