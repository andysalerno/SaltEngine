import { CreatureTakesDamageEvent } from "../message";
import { findBoardCardById } from "../util";

export function handleCreatureTakesDamage(event: CreatureTakesDamageEvent) {
    const cardToDamage = findBoardCardById(event.card_to_damage);

    if (cardToDamage !== undefined) {
        // cardToDamage.current_health -= event.damage;
        cardToDamage.current_health -= 1;
    }
}