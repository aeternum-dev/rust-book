//declarations
use std::io;
use std::thread::sleep;
use std::time::Duration;

//structs for for different mechanisms

//tuple structs for item types
struct Armor (String, i8 );
struct Weapon (String, i8 );
struct Ring (String, i8, i8 );

pub enum Item {
    Armor,
    Weapon,
    Ring
}

pub struct Inventory {
    items: Vec<Item>
}

impl Inventory {
    pub fn add(&mut self, purchased_item: Item) {
        self.items.push(purchased_item);
    }
}



//Player has an inventory element instead of inheriting Inventory
pub struct Player {
    hp: i8,
    dmg: i8,
    arm: i8,
    player_inventory: Inventory
}
    
impl Player {
    fn new() -> Player {
        Player {
            hp: 10, dmg: 0, arm: 0,
            player_inventory: Inventory {items: Vec::new()}
        }
    }
}


    //fn recalculate_stats(&mut self.player_inventory) {
    //    let dmg =
    //    for element in self.player_inventory.Armors {
    //
    //    }
    //}

    
//functions

//calculating damage and armor values
//TODO


//simulating combat
fn battle_sim(mut hp: i8, dmg: i8, arm: i8, mut hp_boss: i8, dmg_boss: i8, arm_boss: i8) -> bool
{
    let is_victorious = loop
    {
        hp_boss = if arm_boss >= dmg {hp_boss - 1} else {hp_boss - (dmg-arm_boss)};
        println!("You deal damage, the boss goes down to {} hp", hp_boss);
        sleep(Duration::from_secs(1));
        if hp_boss < 1 {break true;}

        hp = if arm >= dmg_boss {hp - 1} else {hp - (dmg_boss-arm)};
        println!("The boss deals damage, you go down to {} hp", hp);
        sleep(Duration::from_secs(1));
        if hp < 1 {break false;}
    };

    is_victorious
    
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    //create shop
    let  mut shop_inventory = (
        Armor (String::from("Sturdy Combat Armor"), 10),
        Weapon (String::from("Broadsider"), 11),
        Ring (String::from("Magic ring"), 6, 3)
        
    );
    //print_type_of(&shop_inventory.1);
    //create player
    let mut player1 = Player::new();
        


    //asking user for what items wants to buy
    //TODO: Put in a for cycle

    let mut input_string = String::new();

        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");
            
        
        let item_index: i32 = input_string.trim().parse().unwrap_or(-1);
        
    
    
    
    //add item with the index to player's inventory
    player1.player_inventory.add(shop_inventory.item_index);


    //recalculate damage and armor values


    //battle sim
}