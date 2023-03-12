import { CardDrawn, CardOnBoard } from "./message";

export class BoardSlot {
    _slotNum: number;
    _occupant?: CardOnBoard;

    constructor(slotNum: number) {
        this._slotNum = slotNum;
    }

    set occupant(occupant: CardOnBoard | undefined) {
        this._occupant = occupant;
    }

    get occupant(): CardOnBoard | undefined {
        return this._occupant;
    }

    get getSlotNum(): number {
        return this._slotNum;
    }

    clear() {
        this._occupant = undefined;
    }
}