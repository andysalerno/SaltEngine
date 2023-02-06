import { getContext } from "./js";
import { CardOnBoard } from "./message";
import { endMyTurn, getEndTurnBox, sendSummonFromHandRequest, setCardOnBoardSlot } from "./util";

export function setUpSlots() {
    // Set drag/drop logic for card slots
    const slots = document.querySelectorAll('.board-row.my-side .card-slot')

    for (let slot of slots) {
        slot.addEventListener('dragover', (event) => {
            // Default logic for dragover does not allow dropping.
            event.preventDefault();
        });
        slot.addEventListener('drop', (event) => {
            // At this point we know the div/slot we are dropping on.
            event.preventDefault();

            const target = event.target as HTMLElement;
            const slotNumAttr = target.getAttribute("slotNum");

            if (slotNumAttr !== null) {
                const slotNum = parseInt(slotNumAttr);
                const draggingCard = getContext().draggingCard;
                if (draggingCard !== undefined) {
                    // Send the message to server
                    sendSummonFromHandRequest(draggingCard.id, slotNum);

                    let cardOnBoard: CardOnBoard = draggingCard;

                    // Update visual local state
                    setCardOnBoardSlot(cardOnBoard, slotNum);
                }
            }
        });
    }
}

export function setUpExtraZone() {
    const textBox = document.querySelector(".extra-zone") as HTMLDivElement;

    textBox.addEventListener("click", (event) => {
        const target = event?.target as HTMLDivElement;
        if (target.style.overflow === "visible") {
            target.style.overflow = "hidden";
        } else {
            target.style.overflow = "visible";
        }

    });
}

export function setUpEndTurnButton() {
    const endTurnBox = getEndTurnBox();
    endTurnBox.addEventListener("click", (event) => {
        if (event.target instanceof HTMLDivElement) {
            if (event.target.classList.contains("active")) {
                endMyTurn();
            }
        }
    });

}