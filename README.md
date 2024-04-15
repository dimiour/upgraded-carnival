# upgraded-carnival

#### TODO

* health and damage specific to bullets:
    * make an enumeration or list of enumerations for the classification of objects, possibly renaming them to entities
    * possibly change the physics functions including absorption and drag depending on the class of that entity 
    * add health variable to player/enemy class enum

* weapon changeability:
    * ponder possibilities for weapons

#### DONE

* weapon changeability:
    * create variable (Weapon, Weapon) with boolean to indicate which is in use
    * build a way to indicate which weapon is equipped
    * add ability to switch between weapons on a click
    * add cooldown for shooting

* compartmentalize:
    * create a function in the game implementation for the user interactions
    * switch the assignment of variables in the beginning of the loop into functions for each