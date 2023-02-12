import { CardDrawn, CardId } from "./message";

export class HandSlot {
    _occupant?: CardDrawn;

    set occupant(occupant: CardDrawn | undefined) {
        if (occupant === undefined) {
            console.error("Cannot set an undefined occupant (despite the parameter description");
            return;
        }

        this._occupant = occupant;
    }

    get occupant(): CardDrawn | undefined {
        return this._occupant;
    }

    remove_occupant(): CardDrawn | undefined {
        const previous = this._occupant;
        this._occupant = undefined;

        return previous;
    }
}

export class Hand {
    _slots: Array<HandSlot>;
    _size: number;

    constructor(max_size = 10) {
        this._size = 0;
        this._slots = [];

        for (let i = 0; i < max_size; i++) {
            this._slots.push(new HandSlot());
        }
    }

    get cards(): CardDrawn[] {
        return this._slots.filter(slot => slot._occupant !== undefined).map(slot => slot._occupant!);
    }

    get slots(): HandSlot[] {
        return this._slots;
    }

    add_card(card: CardDrawn) {
        const next_index = this._size;
        const slot = this._slots[next_index];

        slot.occupant = card;

        this._size++;
    }

    remove_at(index: number) {
        if (index > this._size - 1) {
            console.error(`Can't remove card at index ${index} when size is ${this._size}`);
            return;
        }

        // Remove the occupant at the given index.
        const slot = this._slots[index];
        slot.remove_occupant();

        // We removed at index 'n', so indexes 'n+1' and on must move left one
        for (let i = index; i < this._slots.length - 1; i++) {
            const cur_slot = this._slots[i];
            const next_slot = this._slots[i + 1];

            const next = next_slot.remove_occupant();

            if (next !== undefined) {
                cur_slot.occupant = next;
            } else {
                cur_slot.remove_occupant();
            }
        }

        this._size--;
    }

    remove_with_id(id: CardId) {
        const found_index = this._slots.findIndex(slot => slot._occupant?.id.id === id.id);

        if (found_index < 0) {
            console.error(`Did not find card with id ${id.id} in hand.`);
            return;
        }

        this.remove_at(found_index);
    }
}