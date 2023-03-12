import Alpine from 'alpinejs';
import { fadeOutIn } from './animations';
import { cardAttacksTarget } from './attack';
import { BoardSlot } from './boardslot';
import { Hand, HandSlot } from './hand';
import { handleCardDrawn } from './handlers/cardDrawn';
import { handleCreatureDestroyed } from './handlers/creatureDestroyed';
import { handleCreatureTakesDamage } from './handlers/creatureTakesDamage';
import { handlePlayerSummonsCreature } from './handlers/playerSummonsCreature';
import { handleTurnStart } from './handlers/turnStart';
import { CardDrawnEvent, CardDrawn, isHello, isEvent, PlayerStartTurnEvent, PlayerSummonsCreatureClientEvent, CreatureTakesDamageEvent, CreatureDestroyedEvent, CardOnBoard } from './message';
import { setUpEndTurnButton, setUpExtraZone, setUpSlots } from './setup';
import { logGameMessage, parseJson } from './util';

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
    draggingCardToAttack: CardOnBoard | undefined
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
    else if (isEvent(message, "CreatureTakesDamageEvent")) {
        const body = parseJson<CreatureTakesDamageEvent>(message.Event.body);
        handleCreatureTakesDamage(body);
    }
    else if (isEvent(message, "CreatureDestroyedEvent")) {
        const body = parseJson<CreatureDestroyedEvent>(message.Event.body);
        handleCreatureDestroyed(body);
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
        draggingCard: undefined,
        draggingCardToAttack: undefined
    };

    for (let i = 0; i < 12; i++) {
        context.myBoardSide.push(new BoardSlot(i));
        context.enemyBoardSide.push(new BoardSlot(i));
    }

    Alpine.store('gameContext', context);

    Alpine.data('cardhand', (slotNum) => ({
        // We remain bound to the same HandSlot object permanently
        boundTo: context.myHand.slots[slotNum as number],

        get isActive(): boolean {
            const slot = this.boundTo as HandSlot;
            const isMyTurn = getContext().isMyTurn;
            return slot.occupant !== undefined && isMyTurn;
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
        },

        init() {
            this.$watch("boundTo?.occupant?.current_health", (newVal: number, oldVal: number) => {
                if (newVal < oldVal) {
                    this.$el.animate(fadeOutIn, { duration: 500, iterations: 1 });
                }
            });
        },

        clickEnd() {
            const attacker = getContext().draggingCardToAttack;

            if (attacker == undefined) {
                return;
            }

            const target = this.boundTo?.occupant as CardOnBoard | undefined;

            logGameMessage(`${attacker.title} attacks ${target?.title}`);

            if (target === undefined) {
                return;
            }

            cardAttacksTarget(attacker, target);

            getContext().draggingCardToAttack = undefined;
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
            const occupant = this.boundTo.occupant as CardOnBoard | undefined;

            // If we're dragging a creature card, and we are valid target (an empty slot), then we are active.
            if (context.draggingCard !== undefined) {
                if (occupant === undefined) {
                    return true;
                }

                return false;
            }

            // If it is our turn and this slot has an occupant that can attack, then we are active.
            if (context.isMyTurn && occupant !== undefined && occupant.can_attack) {
                return true;
            }

            return false;
        },

        clickStart() {
            logGameMessage("Clicked on board slot...");
            if (this.getIsActive) {
                getContext().draggingCardToAttack = this.boundTo.occupant;
                logGameMessage("...and board slot was active.");
            }
        },

        clickEnd() {
            logGameMessage("Unclicked board slot.");
            getContext().draggingCardToAttack = undefined;
        }
    }));

    (window as any).AlpineContext = Alpine.store('gameContext');

    wsConnect();
});

export function getContext(): Context {
    return Alpine.store('gameContext') as Context;
}

Alpine.start();