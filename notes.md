Back row is for spellcasters/assist, front is for damage

every attack has a text log like old-school RPGs.

## clubs
cats - "hidden"
dogs - mix of support and attack
chess pieces
teachers (math teacher, science teacher...)

"All Seeing Eye Dog"

## keywords
- summon: xyz
- vengeance: xyz
- hidden
- defender
- versatile (back row or front row)

"hidden" creature cards that are "??" until attacked/attack, with price premium. Possibly in a tribe (club) like Cats.

keyword guardians/defenders must be killed before anything behind them.

need better flavor name for front/back row than "attack"/"support"

alternate project name: NetDeck

some AoE cards damage per "slot" and not per "enemy" so deal double damage to 2-wide creatures

front cards are attack-oriented
back cards are support/defense oriented

hero power: summon a 1/1 bug; then, class cards: only summonable while you control a bug


hidden cards are bucketed in costs 3/6/9

impl notes:
for game entities such as "Hand", "Board", "Deck" etc,
there will be a top-level trait like "HandView".
then there will be a struct for the internal game engine representation, like "Hand".
and a simple/flat/serializable struct like "HandPlayerView".
Both internal and external will implement HandView their own way.


all cards get constructed from primitive basics, from a set of templates
that form the basis for all possible cards in the game.
"custom" cards can be created by any player and, a hash can be taken of the definition.
players agree which cards they consider "legal" in the game by having a set of the thumbprints of the cards.
the original set of cards will have hashes that are signed by myself that all clients trust (though allow blacklisting).

iteractive automated tutorial to introduce players to the game, duh

# The protocol

## "Thin"

The server sends actions to clients, and the clients apply the actions to update their own state, and then represent that state graphically.

## "Thick"

The server sends a tuple (action,resulting_state) to clients, and the clients replace their state with the resulting state, and use the action to render some event graphically.  (crazy failure case - the client ends up with a different resulting state than the server's resulting_state)

## "Naive Client"

The server sends property updates to the client, which mutate data, and notifications describing what UI action to render.

Example:

type:visual,event:attack,attacker:fa34,target:4df8
type:update,id:4df8,key:HEALTH,val:7


implementation:
card instances work like this:

// marker trait
trait Card {}

#[derive(Serialize, Deserialize, Card)]
struct AngryBulldog;

clients only see the raw data struct that is serialized/deserialized

server has this:
trait CardLogic {
    /// ... all card logic
}

impl CardLogic for AngryBulldog {
    /// impl...
}

or possibly:
struct CardLogic<T: Card> {
    card: T
}

impl CardLogic<AngryBulldog> {
    /// impl ...
}

or possibly:
struct CardLogic<T: Card> {
    card: T,
    logic: 
}

impl CardLogic<AngryBulldog> {
    /// impl ...
}

client on very beginning:
1. Expect FromServer::Hello
1. Send FromClient::Ready
1. Expect FromServer::GameStart
1. Expect (FromServer::TurnStart | FromServer::Notification)

client on TurnStart:
loop {
    1. Wait for FromServer::WaitingForAction | Prompt (?) | Notification (?)
}