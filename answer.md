# Answers

## Question a 1

no jvm on steamdeck, functionality is determined based on code alone

1. There are two players, either one local and one remote, or two remote players both connecting to a server.
    The give code does not meet this standard, as its either a local+bot or local+remote
    There exist difeculty testing this since the port is hardcodend and it does not allow for swaping in somtingin of our in creation which mocks the remote connection
2. There are 94 cards: 49 center cards, 36 basic cards, and 9 event cards
    a. Center cards consist of 24 region cards, 9 settlements, 7 cities, 9 roads
    b. The two principalities are organized as depicted in Figure 1.
    c. The remaining regions are assigned dice roll as follows and are then shuffled: Field: 3 and 1, Mountain: 4 and 2, Hill: 5 and 1, Forest: 6 and 2, Pasture: 6 and 5, Gold Field: 3 and 2
    d. Regions, settlements, cities, and roads become 4 separate stacks for use during the game.
    e. The basic cards consist of 9 action cards, and 27 settlement/city expansions. These are shuffled and divided into 4 equal draw stacks for use during the game
    f. The event cards stack is shuffled, and then the Yule card is moved to the 4th from the bottom

    this is easy to test as the card class has public static vec which we can exsamine after a call to load basic cards
3. Each player draws 3 cards from a draw stack of their choice
    according to the rules you chose **one** drawstack to pull 3 cards from
    this can be tested in by mocking user input, (this is not easaly done when its hardcoded stdin)
4. The players take turns (random player starts the game):
    a. Roll the event die and the production die (If a brigand attacks, resolve the event before the production, otherwise start with production)
        i. Production: give both player the resource corresponding to the production die (0..3)
        ii. Event: Roll 1 = Brigand, 2=Trade, 3=Celebration, 4=Plentiful Harvest, 5&6=Event card
           1. Brigand: if you have more than 7 resources you lose all your wool and gold
           2. Trade: If you have the trade advantage, you receive 1 resource of your choice from your opponent
           3. Celebration: If you have the most skill points, you alone receive 1 resource of your choice. Otherwise, each player receives 1 resource of his/her choice
           4. Plentiful Harvest: Each player receives 1 resource of his/her choice
           5. Event card: draw a card from the event card stack and resolve the event
    b. Action phase: Play basic cards (either from hand or from the center stack), Trade
    The action phase is difficult to write test for, since its both complex and large, combining this with the difficulty of mocking stdin, this will provide problems
    i. Refer to card descriptions for basic cards, page 18-20, in the rulebook for behavior and cost
    c. Replenish your hand (choose which draw stack to draw from)
    d. Exchange a card from your hand (you may return one card to the bottom of a draw stack, and take one card from a draw stack of your choice, or pay 2 resources to choose 1 card from a stack)
    The code seems to have a search cost for each card you look at, not a fixed cost for looking trugh the stack
5. The winner is announced when a player has 7 or more victory points at the end of his/her turn

## further notes on Testing

Some of the functions would be easy to test, souch as the extract cardsByAttribute
as they have no side effects

but functions like applyEffect have tight integration with current state (given by the active player argument) making it harder to test, as well as reason about, this is not imporved by the fact the function is 300 rows. another thing to mention is that it interects with players feild directly again making it harder to reason about, another thing is that Player is a class so its hard to subsitute with a dummy when testing, or thesting i recomend braking up this function to smaler pices.
