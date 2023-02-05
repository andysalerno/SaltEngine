import { context } from "./js";
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
    const cardsCount = context.myHand.push(card);
    const index = cardsCount - 1;

    const template = document.getElementById("card-hand-template") as HTMLTemplateElement;
    const cloned = template.content.cloneNode(true) as DocumentFragment;
    cloned.firstElementChild?.setAttribute("x-data", `cardhand(${index})`);

    const myHand = document.querySelector<HTMLDivElement>(".my-hand");
    const handSlot = myHand?.querySelectorAll<HTMLDivElement>(".hand-slot")[index];
    handSlot?.appendChild(cloned);
}
