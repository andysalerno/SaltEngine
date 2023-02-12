import Alpine from 'alpinejs';
import { BoardSlot } from './boardslot';
import { Hand, HandSlot } from './hand';
import { CardDrawnEvent, CardDrawn, isHello, isEvent, PlayerStartTurnEvent, FromClient, PlayerSummonsCreatureClientEvent, CardOnBoard } from './message';
import { setUpEndTurnButton, setUpExtraZone, setUpSlots } from './setup';
import { activateEndTurnBox, addCardToHand, deActivateEndTurnBox, getEndTurnBox, logGameMessage, parseJson, removeCardFromHand, sendMessage, setCardOnEnemyBoardSlot } from './util';

type Context = {
    socket: WebSocket | null,
    myId: string | null,
    enemyId: string | null,
    myHand: Hand,
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
    if (event.player_id.guid == getContext().enemyId) {
        logGameMessage(`Enemy summoned: ${event.definition.title}`);

        const definition = event.definition;

        let cardOnBoard: CardOnBoard = {
            title: definition.title,
            current_attack: definition.attack,
            current_cost: definition.cost,
            current_health: definition.health,
            definition: definition,
            id: event.card_id,
        };

        setCardOnEnemyBoardSlot(cardOnBoard, event.target_pos.SlotIndex);
    } else {
        // We immediately placed it on the board when we released the moue button.
        // But now we can remove it from the hand.
        removeCardFromHand(event.card_id);
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
    for (let handCard of context.myHand.cards) {
        if (handCard.current_cost <= context.myMana) {
            // The player can play this card, so add a visual queue.

        }
    }
}

setUpEvents();

document.addEventListener('alpine:init', () => {

    const context: Context = {
        socket: null,
        myId: null,
        enemyId: null,
        myHand: new Hand(),
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

    Alpine.data('cardhand', (slotNum) => ({
        boundTo: context.myHand.slots[slotNum as number],

        get isActive(): boolean {
            const slot = this.boundTo as HandSlot;
            return slot.occupant !== undefined;
        },

        dragStart() {
            getContext().draggingCard = this.boundTo.occupant;
        },

        dragEnd() {
            getContext().draggingCard = undefined;
        }

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