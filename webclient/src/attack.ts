import { Attack, CardDrawn } from "./message";
import { logGameMessage, sendMessage } from "./util";

export function cardAttacksTarget(attacker: CardDrawn, target: CardDrawn) {
    logGameMessage(`${attacker} attacks ${target}`);

    const attack: Attack = {
        attacker_card_id: attacker.id,
        target_card_id: target.id
    };

    sendMessage(attack);
}