import { CardDrawn } from "./message";

export class BoardSlot {
    _slotNum: number;
    _occupant?: CardDrawn;

    constructor(slotNum: number) {
        this._slotNum = slotNum;
    }

    set occupant(occupant: CardDrawn | undefined) {
        this._occupant = occupant;
    }

    get occupant(): CardDrawn | undefined {
        return this._occupant;
    }

    get getSlotNum(): number {
        return this._slotNum;
    }
}