import Alpine from 'alpinejs';
import { BoardSlot } from './boardslot';
import { CardDrawnEvent, CardDrawn, isHello, isEvent, PlayerStartTurnEvent, FromClient } from './message';
import { activateEndTurnBox, addCardToHand, deActivateEndTurnBox, getEndTurnBox, logGameMessage, parseJson, sendMessage } from './util';

type Context = {
    socket: WebSocket | null,
    myId: string | null,
    enemyId: string | null,
    myHand: Array<CardDrawn>,
    myMana: number,
    isMyTurn: boolean,
    myBoardSide: Array<BoardSlot>,

    isDraggingCard: boolean
};

function addCardToSlot() {
    const template = document.getElementById("card-board-template") as HTMLTemplateElement;
    const cloned = template.content.cloneNode(true);

    const slot = document.querySelectorAll(".board-row.my-side-2 > .card-slot")[2];

    slot.appendChild(cloned);
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
    getContext().isMyTurn = false;
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
    const context = getContext();
    context.myMana = event.starting_mana;
    context.isMyTurn = true;

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

setUpEvents();

document.addEventListener('alpine:init', () => {
    Alpine.data('cardhand', (cardNum) => ({
        boundTo: getContext().myHand[cardNum as number],
        isMarkedActive: false,

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
            // return this.isMarkedActive;
            const context = getContext();
            return this.boundTo.current_cost <= context.myMana && context.isMyTurn;
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
        isMyTurn: false,
        myBoardSide: [],
        isDraggingCard: false
    };

    for (let i = 0; i < 12; i++) {
        context.myBoardSide.push(new BoardSlot(i));
    }

    Alpine.store('gameContext', context);

    Alpine.store('myhand', () => ({
        hand: context.myHand
    }));

    Alpine.data('boardSlot', (slotNum) => ({
        boundTo: context.myBoardSide[slotNum as number],

        get getSlotNum(): number {
            return this.boundTo.getSlotNum();
        },

        get getIsActive(): boolean {
            return getContext().isDraggingCard;
        }
    }));

    (window as any).AlpineContext = Alpine.store('gameContext');

    wsConnect();
});

export function getContext(): Context {
    return Alpine.store('gameContext') as Context;
}

Alpine.start();