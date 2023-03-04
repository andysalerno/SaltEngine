import { getContext } from "./js";
import { CardDrawn, CardId, CardOnBoard, FromClient, SummonFromHand } from "./message";

export function getEndTurnBox(): HTMLDivElement {
    return document.querySelector(".endTurnButton") as HTMLDivElement;
}

export function activateEndTurnBox() {
    const endTurnBox = getEndTurnBox();
    endTurnBox.classList.add("active");
}

export function deActivateEndTurnBox() {
    const endTurnBox = getEndTurnBox();
    endTurnBox.classList.remove("active");
}

export function parseJson<T>(json: any): T {
    return JSON.parse(json) as T;
}

export function findBoardCardById(cardId: CardId): CardOnBoard | undefined {
    const context = getContext();

    const mySlot = context.myBoardSide.find(s => s.occupant?.id?.id === cardId?.id);

    if (mySlot !== undefined) {
        return mySlot.occupant;
    }

    const enemySlot = context.enemyBoardSide.find(s => s.occupant?.id?.id === cardId?.id);

    if (enemySlot !== undefined) {
        return enemySlot.occupant;
    }
}

export function addCardToHand(card: CardDrawn) {
    const context = getContext();

    context.myHand.add_card(card);
}

export function removeCardFromHand(id: CardId) {
    const context = getContext();

    // Remove from hand state
    context.myHand.remove_with_id(id);
}

export function setCardOnBoardSlot(card: CardOnBoard, slotNum: number) {
    const context = getContext();

    const slot = context.myBoardSide[slotNum];

    slot.occupant = card;
}

export function setCardOnEnemyBoardSlot(card: CardOnBoard, slotNum: number) {
    const context = getContext();

    const slot = context.enemyBoardSide[slotNum];

    slot.occupant = card;
}

export function logGameMessage(message: string) {
    const rightBox = document.querySelector(".gameEventLog") as HTMLDivElement;

    rightBox.innerHTML += message + "</br>";
}


export function endMyTurn() {
    getContext().isMyTurn = false;
    const message = JSON.stringify(FromClient.EndTurn);
    sendMessage(message);

    deActivateEndTurnBox();
}

export function sendSummonFromHandRequest(card_id: CardId, slotNum: number) {

    const summonFromHand: SummonFromHand = {
        card_id: card_id,
        target_pos: {
            SlotIndex: slotNum
        }
    };

    sendMessage({ SummonFromHand: summonFromHand });
}

export function sendMessage(message: any) {
    let toSend;
    if (typeof message === 'string') {
        toSend = message;
    } else {
        toSend = JSON.stringify(message);
    }

    logGameMessage("Sending: " + toSend);
    getContext().socket?.send(toSend);
}