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

export function addCardToHand(card: CardDrawn) {
    const context = getContext();

    context.myHand.add_card(card);

    // // Attach drag listeners
    // child?.addEventListener('dragstart', () => {
    //     getContext().draggingCard = card;

    // });
    // child?.addEventListener('dragend', () => {
    //     getContext().draggingCard = undefined;
    // });
}

export function removeCardFromHand(id: CardId) {
    const context = getContext();
    const indexInHand = context.myHand.cards.findIndex(card => card.id.id === id.id);

    if (indexInHand < 0) {
        console.error("Expected to find card in hand, but couldn't.");
        return;
    }

    // Remove from DOM
    const handDom = document.querySelector<HTMLDivElement>(".my-hand");
    const handSlot = handDom?.querySelectorAll<HTMLDivElement>(".hand-slot")[indexInHand];

    if (handSlot === undefined) {
        console.error(`Did not find hand slot with index ${indexInHand}`);
        return;
    }

    const handSlotChild = handSlot?.firstChild;

    if (handSlotChild === undefined || handSlotChild === null) {
        console.error("Did not find a child div on the hand slot.");
    } else {
        handSlot?.removeChild(handSlotChild);
    }

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