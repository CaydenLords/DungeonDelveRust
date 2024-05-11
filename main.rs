use text_io::read;
fn main() {
    // Setting up the editable inventory. See functions below for how it works
    let mut inventory:[String;5] = [" ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string()];
    // Game Start, allows the player to skip the tutorial 
    println!("Type START to start");
    let player_input: String = read!();
    if player_input != "SKIP".to_string(){
        tutorial(&mut inventory);
    }
    // Sets up game progress and game over
    let mut game_over = false;
    let mut progress_number = 1;
    // Sets up the overall game loop, running each progress loop until you proceed
    while game_over == false{
        match progress_number{
            // no items
            1 => prog1(&mut inventory, &mut progress_number, &mut game_over),
            // obtained sledgehammer
            2 => prog2(&mut inventory, &mut progress_number, &mut game_over),
            // Opened door
            3 => prog3(&mut inventory, &mut progress_number, &mut game_over),
            // Obtained loot
            4 => prog4(&mut inventory, &mut progress_number, &mut game_over),
            _=> game_over=true
        }
        println!();
    }
    // endgame message
    println!("You have escaped the dungeon with your life, and some loot to boot! However, you can't help but wonder what else lies in the dungeon. ");
    println!("Maybe you'll come back some other time! ")
}


// Tutorial - teaches the basis of the game
fn tutorial(inventory: &mut [String;5]){
    println!("A trapdoor opens, and the ground shakes as sunlight enters the ancient dungons of King Richard the Miserly once again. ");
    println!("Rumors tell of an ancient treasure hidden in these dungeons, and many poor fools have been drawn to their deaths seeking it.");
    println!("However, this has not stopped more adventurers from risking their lives for stories of unlimited wealth.");
    println!("What is the name of this one?");
    let playername: String = read!();
    println!("As {} climbs downwards into the tunnel, their advance is halted by a steel DOOR.",playername);
    println!("Interact with objects in the room by typing words that are in all capitals. Try typing DOOR. ");
    let mut player_input: String = read!();
    while player_input != "DOOR"{
        println!("That doesn't seem to be right. Remember to write in all capitals!");
        player_input = read!();
    }
    println!("Good Work! However, as you reach for the door, it appears to be locked. Looks like you'll need a key. ");
    println!("Look! There's one on the floor! Best pick up that KEY.");
    player_input = read!();
    while player_input != "KEY"{
        println!("That doesn't seem to be right. Remember to write in all capitals!");
        player_input = read!();
    }
    add_to_inventory(inventory, "Tutorial Key");
    display_inventory(inventory);
    println!("This is your inventory. When in the right room, you can type the numbers 1-5 to use an item.");
    println!("Try typing 1 now!");
    player_input = read!();
    while player_input != "1"{
        println!("Remember, the key is in the first slot of your inventory, so type 1 to use it!");
        player_input = read!();
    }
    remove_from_inventory(inventory, "Tutorial Key", 1);
    println!("The key fits perfectly in the lock! The rest of the dungeon is now open to you.");
    display_inventory(inventory);
    println!("As you step into the next room, you hear the door lock behind you. Guess you'll have to find another way out. ");
}

// no items
fn prog1(inventory: &mut [String;5], progress_number: &mut i32, game_over: &mut bool){
    println!("Judging by the grim TOOLS lined up against the wall, this was once a torture chamber. ");
    println!("The BRICKS along the side of the wall seem to have a hole that could lead you to another room.");
    println!("There's also a fancy CHEST against one side of the room. ");
    println!("You need to figure out how to get out of here.");
    display_inventory(inventory);
    let player_input:String = read!();
    if player_input == "TOOLS".to_string(){
            println!("While most of the blood-stained tools don't seem useful right now, one stands out. ");
            println!("It's a sledgehammer! Trying to not think about its previous use, you pick it up.");
            add_to_inventory(inventory, "SLEDGEHAMMER");
            *progress_number = 2;
    }
    if player_input == "BRICKS".to_string(){
        println!("While you can see an exit through the hole in the wall, it's not big enough for you to get through.");
        println!("I guess you'll need some way to get through.");
    }
    if player_input == "CHEST".to_string(){
        println!("The chest sits there, tempting you with what could be in it.");
        println!("However, it's locked and you don't want to try and break it without having a way out.");
    }
}

// obtained sledgehammer
fn prog2(inventory: &mut [String;5], progress_number: &mut i32, game_over: &mut bool){
    println!("The BRICKS along the side of the wall seem to have a hole that could lead you to another room.");
    println!("There's also a fancy CHEST against one side of the room. ");
    println!("You need to figure out how to get out of here.");
    display_inventory(inventory);
    let player_input:String = read!();
    if player_input == "BRICKS".to_string(){
        println!("While you can see an exit through the hole in the wall, it's not big enough for you to get through.");
        println!("I guess you'll need some way to get through.");
    }
    if player_input == "CHEST".to_string(){
        println!("The chest sits there, tempting you with what could be in it.");
        println!("However, it's locked and you don't want to try and break it without having a way out.");
    }
    if player_input == "1".to_string(){
        if remove_from_inventory(inventory, "SLEDGEHAMMER", 1) == true{
            println!("You decide to use the sledgehammer against the bricks in the wall.");
            println!("With some effort, you widen the hole in the wall, leaving a way for you to escape!");
            println!("But you're not leaving empty handed. Time to get some treasure.");
            *progress_number = 3;
        }
        else{
            println!("ERROR");
        }
    }
}

// opened door
fn prog3(inventory: &mut [String;5], progress_number: &mut i32, game_over: &mut bool){
    println!("The HOLE on the other side of the room is your new way out!");
    println!("There's also a fancy CHEST against one side of the room. ");
    println!("You need to find some loot.");
    display_inventory(inventory);
    let player_input:String = read!();
    if player_input == "HOLE".to_string(){
        println!("While you can see an exit through the hole in the wall, you came here for loot, and you're going to get some!");    }
    if player_input == "CHEST".to_string(){
        println!("With your escape secured, you take the time to pick the lock on the chest. Looking inside, you see a redy ruby the size of your hand!");
        println!("Time to go! ");
        add_to_inventory(inventory, "Ruby");
        *progress_number = 4;
    }
}

// obtained loot
fn prog4(inventory: &mut [String;5], progress_number: &mut i32, game_over: &mut bool){
    println!("All that's left is to head out through the HOLE in the wall.");
    display_inventory(inventory);
    let player_input:String = read!();
    if player_input == "HOLE".to_string(){
        println!("As you sneak through the hole, it leads you through a small cave back to the surface.");
        *game_over = true;
    }
}

fn display_inventory(inventory: &mut [String; 5]){
    println!();
    println!("INVENTORY");
    let mut i = 0;
    for item in inventory.iter(){
        i+=1;
        println!("{}. {}", i, item);
    }
    println!();
}

fn add_to_inventory(inventory: &mut [String; 5], pickup: &str){
    let mut picked_up = false;
    let mut i = 0;
    while !picked_up {
            if inventory[i] == " ".to_string() {
                inventory[i] = pickup.to_string();
                picked_up = true;
            }
            i+=1;
    }
}

fn remove_from_inventory(inventory: &mut [String; 5], remove: &str, i: usize)-> bool{
    let mut used: bool = false;
            if inventory[i-1] == remove.to_string() {
                inventory[i-1] = " ".to_string();
                used = true;
            }
            return used;
}
