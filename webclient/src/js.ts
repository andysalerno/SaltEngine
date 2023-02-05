import Alpine from 'alpinejs';
import { CardDrawnEvent, CardDrawn, isHello, isEvent, PlayerStartTurnEvent, FromClient } from './message';
import { activateEndTurnBox, addCardToHand, deActivateEndTurnBox, getEndTurnBox, logGameMessage, parseJson, sendMessage } from './util';

type Context = {
    socket: WebSocket | null,
    myId: string | null,
    enemyId: string | null,
    myHand: Array<CardDrawn>,
    myMana: number
};

function addCardToSlot() {
    const template = document.getElementById("card-board-template") as HTMLTemplateElement;
    const cloned = template.content.cloneNode(true);

    const slot = document.querySelectorAll(".board-row.my-side-2 > .card-slot")[2];

    slot.appendChild(cloned);
}

function removeCardFromSlot() {
    // const slot = document.querySelectorAll(".board-row.my-side-2 > .card-slot")[2];
    // const child = slot.querySelector(".card-board");
    // slot.removeChild(child);
}

function wsConnect() {
    const socket = new WebSocket("ws://127.0.0.1:9001");

    const textBox = document.querySelector(".extra-zone") as HTMLDivElement;

    socket.onopen = () => {
        textBox.innerHTML += "Opened!</br>";
    };

    socket.onmessage = (message) => {
        textBox.innerHTML += message.data + "</br>";

        const parsed = JSON.parse(message.data);

        onMessageReceived(parsed);
    };

    getContext().socket = socket;
}

function setUpEvents() {
    const textBox = document.querySelector(".extra-zone") as HTMLDivElement;

    textBox.addEventListener("click", (event) => {
        const target = event?.target as HTMLDivElement;
        if (target.style.overflow === "visible") {
            target.style.overflow = "hidden";
        } else {
            target.style.overflow = "visible";
        }

    });

    const endTurnBox = getEndTurnBox();
    endTurnBox.addEventListener("click", (event) => {
        if (event.target instanceof HTMLDivElement) {
            if (event.target.classList.contains("active")) {
                endMyTurn();
            }
        }
    });
}

function onMessageReceived(message: any) {
    if (isHello(message)) {
        const context = getContext();
        context.myId = message.Hello[0].guid;
        context.enemyId = message.Hello[1].guid;
    } else if (isEvent(message, "CardDrawnClientEvent")) {
        const body = parseJson<CardDrawnEvent>(message.Event.body);
        handleCardDrawn(body);
    } else if (isEvent(message, "PlayerStartTurnEvent")) {
        const body = parseJson<PlayerStartTurnEvent>(message.Event.body);
        handleTurnStart(body);
    }
}

function endMyTurn() {
    const message = JSON.stringify(FromClient.EndTurn);
    sendMessage(message);

    deActivateEndTurnBox();
}

function handleCardDrawn(event: CardDrawnEvent) {
    if (event.player_id.guid == getContext().myId) {
        logGameMessage("I drew a card.");
        addCardToHand(event.card_drawn.Visible);
    } else {
        logGameMessage("Enemy drew a card.");
    }
}

function handleTurnStart(event: PlayerStartTurnEvent) {
    if (event.player_id.guid === getContext().myId) {
        myTurnStart(event);
    } else {
        enemyTurnStart(event);
    }
}

function myTurnStart(event: PlayerStartTurnEvent) {
    logGameMessage("My turn started. Mana: " + event.starting_mana);
    getContext().myMana = event.starting_mana;

    activateHand();
    activateEndTurnBox();
}

function enemyTurnStart(event: PlayerStartTurnEvent) {
    logGameMessage("Enemy turn started. Mana: " + event.starting_mana);
}

function activateHand() {
    const context = getContext();
    for (let handCard of context.myHand) {
        if (handCard.current_cost <= context.myMana) {
            // The player can play this card, so add a visual queue.

        }
    }
}

addCardToSlot();

removeCardFromSlot();

setUpEvents();

document.addEventListener('alpine:init', () => {
    Alpine.data('cardhand', (cardNum) => ({
        boundTo: getContext().myHand[cardNum as number],

        get getTitle(): string {
            return this.boundTo.title;
        },

        get getAttack(): number {
            return this.boundTo.current_attack;
        },

        get getHealth(): number {
            return this.boundTo.current_health;
        },

        get getIsActive(): boolean {
            return this.boundTo.current_cost <= getContext().myMana;
        },

        get getTitleAndMana(): string {
            return this.boundTo.title + " " + this.boundTo.current_cost;
        }
    }));

    const context: Context = {
        socket: null,
        myId: null,
        enemyId: null,
        myHand: [],
        myMana: 0,
    };
    Alpine.store('gameContext', context);

    Alpine.store('myhand', () => ({
        hand: context.myHand
    }));

    (window as any).AlpineContext = Alpine.store('gameContext');

    wsConnect();
});

export function getContext(): Context {
    return Alpine.store('gameContext') as Context;
}

Alpine.start();