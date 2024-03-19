mod battles;

use std::io::prelude::*;
use std::io;
use std::io::Write;
use std::fs::File;
use std::path::Path;


struct Player {
    name: String,
    class: String,
    level: i16,
    health: i16, //All health and armour values in the game will be 16 bit ints
    armour: i16,
}

struct Inventory {
    gold: i32,
    health_potions: i32,
    items: Vec<u16>,
}


fn main() {

    //Initiating the player and inventory
    let player_name = get_input("Enter your name: ");
    let player_class = get_input("Enter your class (currently only melee)").to_lowercase();

    let mut player = Player {
        name: player_name,
        class: player_class,
        level: 1,
        health: 0,
        armour: 0,
    };

    let mut inventory = Inventory {
        gold: 0,
        health_potions: 0,
        items: vec![],
    };

    if player.class == "melee" {
        player.health = 10;
        player.armour = 5;

    } // TODO Add more player classes here and have validation to insure player enters a class




    battles::battle(player.health, player.armour, player.level);

    //Main game loop
    //loop{

    //}

}

// TODO Need to make this work
fn open_file(){
    let item_path = Path::new("items.csv");
    let display = item_path.display();

    let mut file = match File::open(&item_path){
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}


fn get_input(output_string: &str) -> String {
    println!("{output_string}");
    print!(">");

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input= input.trim();

    return String::from(input);

}