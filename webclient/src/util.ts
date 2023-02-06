import { getContext } from "./js";
import { CardDrawn, FromClient } from "./message";

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
    const cardsCount = getContext().myHand.push(card);
    const index = cardsCount - 1;

    const template = document.getElementById("card-hand-template") as HTMLTemplateElement;
    const cloned = template.content.cloneNode(true) as DocumentFragment;
    const child = cloned.firstElementChild;
    child?.setAttribute("x-data", `cardhand(${index})`);

    // Attach drag listeners
    child?.addEventListener('dragstart', () => {
        getContext().isDraggingCard = true;
        getContext().draggingCard = card;

    });
    child?.addEventListener('dragend', () => {
        getContext().isDraggingCard = false;
        getContext().draggingCard = undefined;
    });

    const myHand = document.querySelector<HTMLDivElement>(".my-hand");
    const handSlot = myHand?.querySelectorAll<HTMLDivElement>(".hand-slot")[index];
    handSlot?.appendChild(cloned);
}

export function setCardOnBoardSlot(card: CardDrawn, slotNum: number) {
    const context = getContext();

    const slot = context.myBoardSide[slotNum];

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