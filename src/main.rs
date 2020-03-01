extern crate clap;

use clap::{Arg, App, AppSettings};


mod input;
use attack::Attack;
mod attack;
mod damage;
use damage::{DamageRoll,DamageDice};

fn main() {
    
       let matches = App::new("D&D dice roller")
                    .version("0.1.0")
                    .author("Marc Guevremont <mg@webs7.com>")
                    .about("Roll like a master")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .arg(Arg::with_name("Attack")
                        .short("a")
                        .long("att")
                        .help("Attack rolls")
                        .takes_value(false))
                    .arg(Arg::with_name("Damage")
                        .short("d")
                        .long("dmg")
                        .help("Damage rolls")
                        .takes_value(false))
                    .get_matches();
    
    let do_attack_roll : Attack;

    if matches.is_present("Attack") {
       do_attack_roll = do_attack();
       if do_attack_roll.get_hits() > 0 || do_attack_roll.get_crits() > 0 {
            do_damage_with_attack(do_attack_roll);
       }
    } else if matches.is_present("Damage"){
        do_damage();
    }
    else
    { 
        do_attack_roll = do_attack();
        if do_attack_roll.get_hits() > 0 || do_attack_roll.get_crits() > 0 {
            do_damage_with_attack(do_attack_roll);
        }
    } 
}

//Each attack can have multiple roll
//Each roll can have multiple dice
fn do_damage_with_attack(a : Attack){
    
    let mut dmg_attacks : Vec<(attack::Outcome,DamageRoll)> = Vec::new();
    let mut dmg_to_copy = DamageRoll::new_empty();
    //Print a message to ask if damage are equal on each attach (y/n)
    let yes_no :Vec<char> = vec!['y','n','c'];     
    let dmg_equal = input::get_char_input_with_default("Roll same damage dice on each attack? (y/n). (c) to exit. Default y.", &yes_no, 'y');
    if dmg_equal == 'c'{
        return 
    }

    let ask_each : bool = dmg_equal == 'n';
    
    if !ask_each {
        dmg_to_copy = DamageRoll::new(create_damage_rolls());
    }
    
    //loop throw hit and critical hits
     for att in a.get_history().into_iter(){
        //if yes ask Damage dice for each attack
        //Ask with history of attack
        match att.1{
            attack::Outcome::Hit => {
                if !ask_each{
                    dmg_attacks.push((attack::Outcome::Hit, dmg_to_copy.copy()));
                }
                else{
                    println!("Damage setting for attack {} {}:", att.0, att.1);
                    dmg_attacks.push((attack::Outcome::Hit, DamageRoll::new(create_damage_rolls())));
                }
            },
            attack::Outcome::Critical => {
                if !ask_each{
                    let crit_dmg = dmg_to_copy.double_dice();
                    dmg_attacks.push((attack::Outcome::Critical, crit_dmg));
                }
                else
                {
                    println!("Damage setting for attack {} {}: Dice will be doubled", att.0, att.1);
                    let crit_dmg = DamageRoll::new(create_damage_rolls()).double_dice();
                    dmg_attacks.push((attack::Outcome::Hit,crit_dmg));
                }
            },
            _ => {}
        }

        
     }   

    //print
    let mut count = 0;
    let mut total = 0;
    let mut total_by_type : [i16; damage::ITEMS_IN_TYPE] = [0;damage::ITEMS_IN_TYPE];  

    for t in dmg_attacks.iter(){
        count += 1;
        match t.0{
            attack::Outcome::Hit => println!("Result for attack {} which is a {}", count,t.0),
            attack::Outcome::Critical => println!("Result for attack {} which is a {}", count,t.0),
            _ => {}
        }

        let mut tt = t.1.copy();
        tt.roll();

        for x in 0..damage::ITEMS_IN_TYPE{
            total_by_type[x] += tt.get_dmg_by_type()[x];
        }

        total += tt.get_total();

        println!("{:?}", tt );
    }
    damage::display_dmg_type(total_by_type);

    println!("Total: {}", total);
}

fn do_damage(){
    let mut dmg = DamageRoll::new(create_damage_rolls());
    dmg.roll();
    println!("{:?}", dmg );
}


fn create_damage_roll() -> DamageDice{
    let mut damage_type : i16;
    let dmg_to_enum : damage::Type;
    loop{
        damage_type = input::get_i16_input_with_default("\n 
Type of damage? \n 
1)Slashing, 2)Piercing, 3)Bludgeoning, 4)Poison,   5)Acid,  6)Fire,  7)Cold, \n       
8)Radiant,  9)Necrotic, 10)Lightning   11)Thunder, 12)Force, 13)Psychic \n
Press Enter to ignore ", 0);

        match damage_type{
            0 =>  { dmg_to_enum = damage::Type::Ignored; break;},
            1 =>  { dmg_to_enum = damage::Type::Slashing; break;},
            2 =>  { dmg_to_enum = damage::Type::Piercing; break;},
            3 =>  { dmg_to_enum = damage::Type::Bludgeoning; break;},
            4 =>  { dmg_to_enum = damage::Type::Poison; break;},
            5 =>  { dmg_to_enum = damage::Type::Acid; break;},
            6 =>  { dmg_to_enum = damage::Type::Fire; break;},
            7 =>  { dmg_to_enum = damage::Type::Cold; break;},
            8 =>  { dmg_to_enum = damage::Type::Radiant; break;},
            9 =>  { dmg_to_enum = damage::Type::Necrotic; break;},
            10 => { dmg_to_enum = damage::Type::Lightning; break;},
            11 => { dmg_to_enum = damage::Type::Thunder; break;},
            12 => { dmg_to_enum = damage::Type::Force; break;},
            13 => { dmg_to_enum = damage::Type::Psychic; break;},
            _ => {}
        }
    }
    let rolls = input::get_i16_input("Number of dice?");
    let mut die_type : i16;
    let chosen_die : i16;
    loop{
        die_type = input::get_i16_input("Type of dice? \n
1)d4, 2)d6, 3)d8, 4)d10, 5)d12, 6)d20, 7)d100");
        match die_type{
            1 => {chosen_die = 4;  break;},
            2 => {chosen_die = 6;  break;}
            3 => {chosen_die = 8;  break;}
            4 => {chosen_die = 10; break;}
            5 => {chosen_die = 12; break;}
            6 => {chosen_die = 20; break;}
            7 => {chosen_die = 100; break;}
            _ => {}
        }
    }   
    let weapon_modifier = input::get_i16_input_with_default("Damage modifier? Default 0", 0);

    DamageDice::new(dmg_to_enum, rolls, chosen_die, weapon_modifier)
}

fn create_damage_rolls() -> Vec<DamageDice>{
    //Ask for new dice input until until user say no
    
    let mut damage_rolls : Vec<DamageDice> = Vec::new();
    
    loop{
        damage_rolls.push(create_damage_roll());
        let yes_no :Vec<char> = vec!['y','n','r'];
        
        let other_roll = input::get_char_input_with_default("Add more dice? (y/n). To reset(r). Default n.", &yes_no, 'n');

        if other_roll == 'n'{
            break;
        } 
        else if other_roll == 'r'{
            damage_rolls = Vec::new();
        }
    }

    damage_rolls
}

fn do_attack() -> Attack{
        
    let num_attack  = input::get_i16_input("Number of attack?");
    let armor_class = input::get_i16_input("Armor class to beat?");
    let weapon_modifier = input::get_i16_input("Weapon modifier?");

    let attack_type_choice :Vec<char> = vec!['n','a','d'];
    let yes_no :Vec<char> = vec!['y','n'];
    let attack_type = input::get_char_input_with_default("Type of attack? Normal(n) , Advantage(a) , Disadvantage(d).  Default n", &attack_type_choice, 'n');
    let c_hit  = input::get_i16_input_with_default("Critical Hit On? Default 20", 20);
    let stop_on_fumble = input::get_char_input_with_default("Stop the attacks after a fumble? (y/n) Default y.", &yes_no, 'y');

    let type_of_attack : attack::Type;
    let stf = if stop_on_fumble == 'y'{true} else {false};

    match attack_type {
        'n' => {type_of_attack = attack::Type::Normal},
        'd' => {type_of_attack = attack::Type::Disadvantage},
        'a' => {type_of_attack = attack::Type::Advantage},
        _   => {type_of_attack = attack::Type::Normal}
    }

    let a : Attack = Attack::new(type_of_attack, num_attack, armor_class, weapon_modifier, c_hit, stf);
    
    println!("{:?}", a); a
}


