import Alpine from 'alpinejs';
import { BoardSlot } from './boardslot';
import { CardDrawnEvent, CardDrawn, isHello, isEvent, PlayerStartTurnEvent, FromClient, PlayerSummonsCreatureClientEvent } from './message';
import { setUpEndTurnButton, setUpExtraZone, setUpSlots } from './setup';
import { activateEndTurnBox, addCardToHand, deActivateEndTurnBox, getEndTurnBox, logGameMessage, parseJson, sendMessage } from './util';

type Context = {
    socket: WebSocket | null,
    myId: string | null,
    enemyId: string | null,
    myHand: Array<CardDrawn>,
    myMana: number,
    isMyTurn: boolean,
    myBoardSide: Array<BoardSlot>,
    enemyBoardSide: Array<BoardSlot>,

    draggingCard: CardDrawn | undefined
};

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
    setUpExtraZone();
    setUpSlots();
    setUpEndTurnButton();
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
    else if (isEvent(message, "PlayerSummonsCreatureClientEvent")) {
        const body = parseJson<PlayerSummonsCreatureClientEvent>(message.Event.body);
        handlePlayerSummonsCreature(body);
    }
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

function handlePlayerSummonsCreature(event: PlayerSummonsCreatureClientEvent) {
    if (event.player_id.player_id.guid == getContext().enemyId) {

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
        enemyBoardSide: [],
        draggingCard: undefined
    };

    for (let i = 0; i < 12; i++) {
        context.myBoardSide.push(new BoardSlot(i));
        context.enemyBoardSide.push(new BoardSlot(i));
    }

    Alpine.store('gameContext', context);

    Alpine.store('myhand', () => ({
        hand: context.myHand
    }));

    Alpine.data('enemyBoardSlot', (slotNum) => ({
        boundTo: context.enemyBoardSide[slotNum as number],

        get getSlotNum(): number {
            return this.boundTo.getSlotNum();
        },

        get title(): string {
            return this.boundTo.occupant.title;
        },

        get getIsActive(): boolean {
            return false;
        }
    }));

    Alpine.data('boardSlot', (slotNum) => ({
        boundTo: context.myBoardSide[slotNum as number],

        get getSlotNum(): number {
            return this.boundTo.getSlotNum();
        },

        get title(): string {
            return this.boundTo.occupant.title;
        },

        get getIsActive(): boolean {
            const context = getContext();
            return context.draggingCard !== undefined
                && this.boundTo.occupant === undefined;
        }
    }));

    (window as any).AlpineContext = Alpine.store('gameContext');

    wsConnect();
});

export function getContext(): Context {
    return Alpine.store('gameContext') as Context;
}

Alpine.start();