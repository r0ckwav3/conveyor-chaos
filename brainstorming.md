# Game concept:

Block manipulation game a la opus magnum

Primary tool is a tile that will push anything that lands on top of it

Rotation?
Rotation can be done with a pivot + a pusher

How about combining or splitting?
It would be idiomatic to use two arrows pointing apart to split things

Priority systems of arrows:
 * If there are two arrows trying to push the same object:
   * Only consider top priority arrows
   * if there are two arrows of top priority:
     * in opposite directions - try to split
     * in the same direction - just push in that direction
     * in perpendicular directions - either see if one has more pushing power or just say invalid

Combining:
 * two adjacent objects try to go into each other (importantly, not just one going into the other)
 * see examples for how to make this not break
 * bad combining cause "grid misalignment"

Examples:
<^>v push tiles
<1 <2 <3 priorities (1 is top)
<# has a thing on top
I is input
O is output

Valid split:
<#>#
<#>#

Invalid split:

<#>#
 # #

2x2 split mechanism:

OO    OO
^2<2>2^2
V2<2>2v2
OO^1^1OO
  ^1^1
  ^I^I
  ^I^I

1x2 merger:

  OOOO
>1<2^1
^I^I


# UI brainstorming:

I need the player to be able to do:
 * Move around the board space
 * Place tiles of multiple varieties
 * Rotate tiles
 * Place products and inputs
 * move products and inputs
 * rotate products and inputs
 * Start the level

Potential input mechanisms:
 * Drag elements onto the board
   * alternatively, click on something and then click on the board to place it
 * Drag the board
   * use a key combo to drag the board like right-click or shift click
 * click a tile to select what it should be
 * hover and use r or shift-r to rotate a tile
 * click a product to select it for movement
 * select something and then hit delete to remove it
 * use keybindings for specific tiles, e.g. 1,2,3,4 to specify four tiletypes

Set 1:
 * Drag on board - move board
 * Click on board
   * select something (block objects first)
     * if you're hovering over something already selected, attempt to cycle:
       * block object
       * tile
       * nothing
 * when selecting a tile:
   * r to rotate
 * drag something onto the board
   * place either block object or tile

Set 2:
 * Drag on board - move board
 * Hover over tile + (r or shift + r)
   * Rotate
 * hover over tile + some key (maybe t?)
   * clear tile
 * click on tile in sidebar - select tile
 * click on board:
   * if holding tile place it
   * if shift is held, keep it in the hand
 * click on product or input:
   * remove it from the board if it exists
   * select it

I'm currently leaning towards set 2

Also, can you rotare an input or output while inputting it?

# Aesthetics Brainstorming
Has to explain:
 * What are the tiles?
   * Probably the easiest to explain, either maglev or conveyors or magic runes
 * What are the blocks
   * This is actually a very intesting question, because we're making very crude shapes most of the time
   * In addition, there's currently only one type of block, but that can be changed
   * This makes me lean away from making human-scale things
   * Vague ideas:
     * Atoms or similar (opus magnum and molek-syntez both do this though)
     * Pre-fab housing
     * non-representational blobs, just a puzzle game with no explaination
       * In that case I could go with a kidna sketchbook look for everything else
       * Probably have colored blobs as well?
     * Magical essences or something (I'm also worried it'll end up looking too much like opus magnum)
     * Spaceships?? I would need triangular peices to make it look aerodynamic though
       * What is the minimum number of peices? Square, triangle, thruster
     * Piping?
       * very few peices needed, but I feel like you can get a lot of milage out of it
       * lends well to a very warehouse-y industrial aesthetic as well, which is sorta what I'm already going for
       * Could also lean steampunk
 * Why are we automating?
   * This is very large or very small, so cannot be done by a person
   * This will be done a lot so automation is neccesary
   * This is not a physical process that a person can do
