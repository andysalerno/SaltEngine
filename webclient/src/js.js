// The global context.
const context = {
    socket: null,
    myId: null,
    enemyId: null,
    myHand: []
};

function addCardToSlot() {
    const template = document.getElementById("card-board-template");
    const cloned = template.content.cloneNode(true);

    const slot = document.querySelectorAll(".board-row.my-side-2 > .card-slot")[2];

    slot.appendChild(cloned);
}

function removeCardFromSlot() {
    // const slot = document.querySelectorAll(".board-row.my-side-2 > .card-slot")[2];
    // const child = slot.querySelector(".card-board");
    // slot.removeChild(child);
}

function wsConnect() {
    const socket = new WebSocket("ws://127.0.0.1:9001");

    const textBox = document.querySelector(".extra-zone");

    socket.onopen = () => {
        textBox.innerHTML += "Opened!</br>";
    };

    socket.onmessage = (message) => {
        textBox.innerHTML += message.data + "</br>";

        const parsed = JSON.parse(message.data);

        onMessageReceived(parsed);
    };

    context.socket = socket;
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

function logGameMessage(message) {
    const rightBox = document.querySelectorAll(".extra-zone")[1];

    rightBox.innerHTML += message + "</br>";
}

function onMessageReceived(message) {
    if (message.Hello) {
        context.myId = message.Hello[0].guid;
        context.enemyId = message.Hello[1].guid;
    } else if (message.Event.kind === "CardDrawnClientEvent") {
        const body = JSON.parse(message.Event.body);
        handleCardDrawn(body);
    }
}

function handleCardDrawn(event) {
    if (event.player_id.guid == context.myId) {
        logGameMessage("I drew a card.");
        const cardsCount = context.myHand.push(event.card_drawn.Visible);
        const index = cardsCount - 1;

        const template = document.getElementById("card-hand-template");
        const cloned = template.content.cloneNode(true);

        const myHand = document.querySelector(".my-hand");
        const handSlot = myHand.querySelectorAll(".hand-slot")[index];
        handSlot.appendChild(cloned);
    } else {
        logGameMessage("Enemy drew a card.");
    }
}

addCardToSlot();

removeCardFromSlot();

setUpEvents();

wsConnect();