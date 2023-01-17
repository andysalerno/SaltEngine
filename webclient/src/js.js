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

function wsConnect() {
    const socket = new WebSocket("ws://127.0.0.1:9001");

    const textBox = document.querySelector(".extra-zone");

    socket.onopen = (event) => {
        textBox.innerHTML += "Opened!</br>";
    };

    socket.onmessage = (message) => {
        textBox.innerHTML += message.data + "</br>";
    };

}

function setUpEvents() {
    const textBox = document.querySelector(".extra-zone");

    textBox.addEventListener("click", (event) => {
        if (event.target.style.overflow === "visible") {
            event.target.style.overflow = "hidden";
        } else {
            event.target.style.overflow = "visible";
        }

    });
}

addCardToSlot();

removeCardFromSlot();

setUpEvents();

wsConnect();