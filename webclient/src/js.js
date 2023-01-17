function addCardToSlot() {
    const template = document.getElementById("card-board-template");
    const cloned = template.content.cloneNode(true);

    const slot = document.querySelectorAll(".board-row.my-side-2 > .card-slot")[2];

    slot.appendChild(cloned);
}

function removeCardFromSlot() {
    const slot = document.querySelectorAll(".board-row.my-side-2 > .card-slot")[2];
    const child = slot.querySelector(".card-board");

    // slot.removeChild(child);
}

addCardToSlot();

removeCardFromSlot();