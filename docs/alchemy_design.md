# Alchemy System
The alchemy system is the absolute core of this game.
The game is built around the player's ability to farm/gather ingredients, brew potions, refine them, and sell/gift/use them.
Here we'll document the broad goals of this system and some possible solutions to meet these goals.
Design changes, if they're significant enough, should also probably be documented here.

Warning: this document contains spoilers for the game.
Discovering and experimenting with these mechanics is part of the fun!

## Design Goals
The final design of the alchemy system should at least meet the following criteria:
1. Systemic/emergent - no scripted recipes
2. Technical - takes some physical skill, dexterity, mastery
3. Meritocratic - Better results require more skill, more complicated recipes, rarer ingredients, etc
4. Obscure - internal workings not revealed to player.
   Though, they may be discoverable through experimentation, or progressing the story/side quests.

Why these criteria?
Well this is personally the exact kind of system that I love playing with in games.
Deep systems with some elements of mastery and mystery.

The following sections of this document go over some other "goals", though these are mostly just in service of the four above.
Consequently, we call them ideas instead of goals.
They also go into more detail and flesh out the system much more than these main design goals.
The ultimate design may incorporate elements from any combination of these ideas.
In other words, they are not mutually exclusive.

## Idea: Chemistry-based
This seems to be as good a place as any to start.
The idea is simple, there are some basic alchemical elements and the way that these elements interact with eachother is not too far off from the interactions on the periodic table.
These elements each contain a certain number of "altons" (like protons but magical).
These elements can combine to form alchemical compounds.
These compounds can react with each other in the brewing process, resulting in new compounds.

Each compound has its own set of effects when consumed in a potion.
Most potions will have multiple compounds in them, giving a potion multiple effects of varying strength.
Though, an effect in one compound may work to "cancel out" some effect in another compound.
This effect-canceling mechanic may allow the player to make a potion have less effects, which may be desirable if they want it to be "pure".

Note: there has been some thought into applying potions or the alchemy system in other ways than consumption.
For example, perhaps compounds have related affects when applied to a tool or item through enchantments.
Maybe the player's broomstick requires some flight or lightweight enchantment to ride.

Another note: consider other ways effects might interact with each other, other than canceling.

Similar to potions, basic ingredients also contain compounds.
Consuming these will apply the effects of those compounds, but probably to a lesser degree than a potion.
Or, at the very least, the compounds present in an ingredient aren't as effective as those only achievable through brewing.

Compound-based ingredients could even allow for some nice depth in the farming mechanics.
Ingredients could be affected by the presence of weeds simply by altering their compound structure.
Or, even better, the player could play around with breeding their ingredients.
Are GMOs cottage-core?

## Idea: Alchemy-based
Perhaps some of the theory and practice of potion making could be based on real-life alchemy/potion making.
Not necessarily the alchemy interested in the transmutation of metals, but more the medicinal alchemy.
There's some interesting information about historical alchemy on wikipedia.
The most relevant articles outside of the article on Alchemy itself seem to be Spagyric, Herbal Medicine, Potion, and Extract.
We could also probably find additional inspiration in Rennaissance Magic and Necromancy (not just about death apparently).

### Techniques
Alchemical procedures include:
- fermentation (conversion to alcohol w/ microorganisms)
- distillation (boiling then collecting the condensation)
- absorption (steeping material in a solvent)
- expression (juicing)

Absorption may be split up into:
- infusion (steeping in hot water)
- decoction (long-term boiled extracts for harder materials)
- maceration (softening material w/o heat in water or oil)

We may be able to take inspiration from all of these techniques when developing the alchemy.
Of course, we are also interested in just plain alchemical reaction when combining ingredients/extracts/tinctures/potions.

### Tools
Originally the plan was just to stick to plain alchemical reactions in the cauldron.
However, taking inspiration from real-world alchemy, we may be able to add some more "vectors of input".

Some old alchemical tools for metallurgy include:
- crucibles (for carrying smaller amounts of hot liquid)
- bellows (for adding oxygen to fires)
- filters
- cauldron
- alembic (for distillation)

There are probably other tools that are more applicable to medicinal alchemy.
A Mortar and Pestle come to mind.
TODO: research old pharmaceutical tools.
