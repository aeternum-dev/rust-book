use std::thread::sleep;
use std::time::Duration;

//battle_sim(hp, dmg, arm, hp_boss, dmg_boss, arm_boss)
//  while true
//      hp_boss = hp_boss-(dmg-arm_boss) //OR IF ARMOR IS TOO HIGH
//      if hp_boss < 1:
//          break
//      hp = hp-(dmg_boss-arm)
//      if hp_boss < 1:
//          break


//Stuff I am going to need:
// loop cycle
//logical value assignment
//let number = if condition { 5 } else { 6 };
//cmp ordering shitto
// if other guy's armor is greater or equal, then 1 final damage, else the difference


fn battle_sim(mut hp: u8, dmg: u8, arm: u8, mut hp_boss: u8, dmg_boss: u8, arm_boss: u8) -> bool
{
    let player_victorious = loop
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

    player_victorious
    //and basically this loop gives us the final outcome
    //let's
}

fn main() {
    println!("{}",battle_sim(8,5,5,12,7,2));
    println!("Hello, world!");
}
