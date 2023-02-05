export class BoardSlot {
    _slotNum: number;

    constructor(slotNum: number) {
        this._slotNum = slotNum;
    }

    get getSlotNum(): number {
        return this._slotNum;
    }
}