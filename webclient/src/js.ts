import Alpine from 'alpinejs';
import { CardDrawnEvent, CardDrawn, isHello, isEvent, PlayerStartTurnEvent, FromClient } from './message';
import { activateEndTurnBox, addCardToHand, deActivateEndTurnBox, getEndTurnBox, logGameMessage, parseJson, sendMessage } from './util';

type Context = {
    socket: WebSocket | null,
    myId: string | null,
    enemyId: string | null,
    myHand: Array<CardDrawn>
};

// The global context.
export const context: Context = {
    socket: null,
    myId: null,
    enemyId: null,
    myHand: [],
};

(window as any).Context = context;

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

    context.socket = socket;
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
        endMyTurn();
    });
}


function onMessageReceived(message: any) {
    if (isHello(message)) {
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
    if (event.player_id.guid == context.myId) {
        logGameMessage("I drew a card.");
        addCardToHand(event.card_drawn.Visible);
    } else {
        logGameMessage("Enemy drew a card.");
    }
}

function handleTurnStart(event: PlayerStartTurnEvent) {
    if (event.player_id.guid === context.myId) {
        myTurnStart(event);
    } else {
        enemyTurnStart(event);
    }
}

function myTurnStart(event: PlayerStartTurnEvent) {
    logGameMessage("My turn started. Mana: " + event.starting_mana);
    activateHand();
    activateEndTurnBox();
}

function enemyTurnStart(event: PlayerStartTurnEvent) {
    logGameMessage("Enemy turn started. Mana: " + event.starting_mana);
}

function activateHand() { }

addCardToSlot();

removeCardFromSlot();

setUpEvents();

wsConnect();

document.addEventListener('alpine:init', () => {
    Alpine.data('cardhand', (cardNum) => ({
        title: 'some title',
        attack: cardNum,
        cardNum: cardNum,
        health: 5,

        get getTitle(): string {
            const card = context.myHand[this.cardNum as number];
            return card.title;
        },

        get getTitleAndMana(): string {
            const card = context.myHand[this.cardNum as number];
            return card.title + " " + card.current_cost;
        }
    }));

    Alpine.data('myhandslot', (slotIndex) => ({
        index: slotIndex,

    }));

    Alpine.store('myhand', () => ({
        hand: context.myHand
    }));
});

Alpine.start();