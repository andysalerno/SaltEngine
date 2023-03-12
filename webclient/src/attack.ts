import { Attack, CardDrawn, CardOnBoard } from "./message";
import { logGameMessage, sendMessage } from "./util";

export function cardAttacksTarget(attacker: CardOnBoard, target: CardOnBoard) {
    logGameMessage(`${attacker} attacks ${target}`);

    const attack: Attack = {
        attacker_card_id: attacker.id,
        target_card_id: target.id
    };

    sendMessage({ Attack: attack });

    attacker.can_attack = false;
}