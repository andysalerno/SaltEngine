// {"Event":{"kind":"CardDrawnClientEvent","body":"{\"player_id\":{\"guid\":\"1f4edd7a-3b84-49be-ac0b-9e7d726b286f\"},\"card_drawn\":{\"Visible\":{\"id\":{\"id\":\"89d8c455-2e42-4b22-a483-64265c65f9a9\"},\"definition\":{\"title\":\"Sleeping Dog\",\"cost\":1,\"attack\":0,\"health\":3},\"title\":\"Sleeping Dog\",\"current_cost\":1,\"current_attack\":0,\"current_health\":3}}}"}}
// {"SummonFromHand":{"card_id":{"id":"a2b34d4a-fcce-4b4a-8500-e49b771b2af0"},"target_pos":{"SlotIndex":7}}}
// "Event":{"kind":"PlayerSummonsCreatureEvent","body":"{\"player_id\":{\"guid\":\"113e9d90-d1e3-4c0b-b47c-1370c63d29e7\"},\"card_id\":{\"id\":\"71bc93a8-5899-4790-8d53-22891036a1d6\"},\"target_pos\":{\"SlotIndex\":1}}"}
// {"Event":{"kind":"PlayerSummonsCreatureClientEvent","body":"{\"player_id\":{\"guid\":\"98758e6a-7b5a-4c58-8e23-0697a6836e1d\"},\"card_id\":{\"id\":\"41ce8bf2-9c5c-4fc2-b360-273898a33a96\"},\"target_pos\":{\"SlotIndex\":8},\"definition\":{\"title\":\"Sleeping Dog\",\"cost\":1,\"attack\":0,\"health\":3}}"}}

export interface Id {
    guid: string;
}

export type PlayerId = {
    player_id: Id
}

export interface CardDrawnEvent {
    player_id: Id;
    card_drawn: Visible<CardDrawn>;
}

export interface PlayerStartTurnEvent {
    player_id: Id;
    starting_mana: number
}

export enum FromClient {
    EndTurn = "EndTurn",
}

export type CardId = {
    id: string
}

export type GamePos = {
    SlotIndex: number
}

export type SummonFromHand = {
    card_id: CardId,
    target_pos: GamePos
}

export type CardDefinition = {
    title: string,
    cost: number,
    attack: number,
    health: number
}

export interface CardDrawn {
    title: string;
    current_attack: number;
    current_cost: number;
    current_health: number;
    definition: any;
    id: CardId;
}

export type PlayerSummonsCreatureClientEvent = {
    player_id: PlayerId,
    card_id: CardId,
    target_pos: GamePos,
    definition: CardDefinition
}

export interface Event<TEvent> {
    kind: string;
    body: TEvent;
}

export interface EventMessage<TEvent> {
    Event: Event<TEvent>;
};

export interface HelloMessage {
    Hello: Array<{ guid: string }>;
}

export interface Visible<T> {
    Visible: T;
}

export function isHello(message: any): message is HelloMessage {
    return !!(message as HelloMessage).Hello;
}

export function isEvent(message: any, name: string): boolean {
    const inner = (message as any).Event;

    return inner.kind === name;
} 