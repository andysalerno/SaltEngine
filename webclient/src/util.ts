import { getContext } from "./js";
import { CardDrawn } from "./message";

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
    cloned.firstElementChild?.setAttribute("x-data", `cardhand(${index})`);
    // cloned.firstElementChild?.setAttribute("x-data", `$store.gameContext`);
    // cloned.firstElementChild?.setAttribute("x-data",
    //     `{
    //     health: 10,
    //     attack: 5
    //  }`);

    const myHand = document.querySelector<HTMLDivElement>(".my-hand");
    const handSlot = myHand?.querySelectorAll<HTMLDivElement>(".hand-slot")[index];
    handSlot?.appendChild(cloned);
}

export function logGameMessage(message: string) {
    const rightBox = document.querySelector(".gameEventLog") as HTMLDivElement;

    rightBox.innerHTML += message + "</br>";
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