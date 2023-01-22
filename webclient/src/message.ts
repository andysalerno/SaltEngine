// {"Event":{"kind":"CardDrawnClientEvent","body":"{\"player_id\":{\"guid\":\"1f4edd7a-3b84-49be-ac0b-9e7d726b286f\"},\"card_drawn\":{\"Visible\":{\"id\":{\"id\":\"89d8c455-2e42-4b22-a483-64265c65f9a9\"},\"definition\":{\"title\":\"Sleeping Dog\",\"cost\":1,\"attack\":0,\"health\":3},\"title\":\"Sleeping Dog\",\"current_cost\":1,\"current_attack\":0,\"current_health\":3}}}"}}

export interface Id {
    guid: string;
}

export interface CardDrawnEvent {
    player_id: Id;
    card_drawn: Visible<CardDrawn>;
}


export interface CardDrawn {
    title: string;
    current_attack: number;
    current_cost: number;
    current_health: number;
    definition: any;
    id: { id: string };
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