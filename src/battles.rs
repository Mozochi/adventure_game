//pub mod main.rs
use rand::Rng;

pub fn battle(player_hp: i16, player_armour: i16, player_lvl: i16){

    let enemy = enemy_gen(player_lvl);



    //Battle loop
    //loop{

   // }

}

//                                Enemy Name, Enemy Health, Enemy Level
fn enemy_gen(player_lvl: i16) -> (String, i16, i16) {

    let enemy_name:String = String::from("Temp");
    let enemy_lvl = rand::thread_rng().gen_range(player_lvl..=(player_lvl+2));

    let enemy_health = (enemy_lvl * 2)-2;


    return (enemy_name, enemy_health ,enemy_lvl,);
}