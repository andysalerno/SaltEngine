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