extern crate rand;
use rand::distributions::{Distribution, Uniform};




use std::fmt;

/*
 * Each Damage Dice may have multiple dice. For example 6d6 +2.
 * Each Damage Roll may have multiple Damage Dice. For example 2d6 percing + 1d6 fire +3
 */

#[derive(Copy, Clone)]
pub enum Type {
    Ignored = 0,
    Slashing,
    Piercing,
    Bludgeoning,
    Poison,
    Acid,
    Fire,
    Cold,
    Radiant,
    Necrotic,
    Lightning,
    Thunder,
    Force,
    Psychic,
}

pub const ITEMS_IN_TYPE: usize = 14;

//Represent a single roll of damage
pub struct DamageDice{
    Type     : Type,
    rolls    : i16,
    die      : i16,
    modifier : i16,
    history : Vec<i16>,
    total    : i16
}
//List of Damage Dice
pub struct DamageRoll{
    dmg_by_type : [i16;ITEMS_IN_TYPE],
    total       : i16,
    modifier    : i16,
    dmg_rolls : Vec<DamageDice>
}


impl DamageDice {
   
    pub fn new(t:Type, r: i16, die: i16, modif: i16) -> DamageDice{

        DamageDice{
            Type : t,
            rolls : r,
            die : die,
            modifier : modif,
            history : Vec::new(),
            total : 0
        }
    }

    pub fn roll (&mut self) {
        let mut total = 0;
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..self.die+1);
        let dice_roll = self.rolls;
        

        println!("Dice roll: {}d{} {}  ", dice_roll, self.die,
        if self.modifier > 0 {format!("{}{}", "+", self.modifier)} else {format!("")});

        
        for _ in 0..dice_roll{    
            let throw = die.sample(&mut rng);
            print!("{}, ", throw);
            total = total + throw;
            self.history.push(throw as i16);
        }
        println!("");
        self.total = total + self.modifier;
    }

   
}

impl DamageRoll{
    pub fn new(dmg_roll : Vec<DamageDice>) ->  DamageRoll{
       DamageRoll{
           dmg_by_type :  [0;ITEMS_IN_TYPE],
           total       : 0,
           modifier     : 0,
           dmg_rolls : dmg_roll
       }
    }

    pub fn new_empty() -> DamageRoll{
        DamageRoll{
           dmg_by_type : [0;ITEMS_IN_TYPE],
           total       : 0,
           modifier    : 0,
           dmg_rolls : Vec::new()
        }
    }

    //Double dice without modifier for critical hit
    pub fn double_dice(&mut self) -> DamageRoll{
        let mut new_vec : Vec<DamageDice> = Vec::new();
        for i in self.dmg_rolls.iter(){

            new_vec.push(DamageDice::new(i.Type,i.rolls,i.die,0));
            new_vec.push(DamageDice::new(i.Type,i.rolls,i.die,i.modifier));
        }
        DamageRoll{
            dmg_by_type : [0;ITEMS_IN_TYPE],
            total       : 0,
            modifier    : 0,
            dmg_rolls : new_vec
        }
    }

    pub fn copy(&self) -> DamageRoll {
        let mut new_vec : Vec<DamageDice> = Vec::new();

        for i in self.dmg_rolls.iter(){
            new_vec.push(DamageDice::new(i.Type,i.rolls,i.die,i.modifier));
        }
        DamageRoll{
           dmg_by_type : [0;ITEMS_IN_TYPE],
           total       : 0,
           modifier    : 0,
           dmg_rolls : new_vec
        }
    }

    
    pub fn roll(& mut self){
        
        for dice in self.dmg_rolls.iter_mut(){
            dice.roll();
            self.total += dice.total;
            self.dmg_by_type[dice.Type as usize] += dice.total;
            self.modifier = if dice.modifier > 0 {dice.modifier} else {0}; 
        }
    }
    pub fn get_dmg_by_type(&self) -> [i16;ITEMS_IN_TYPE]{
        self.dmg_by_type
    }
    pub fn get_total(&self) -> i16 {
        self.total
    }
}

pub fn display_dmg_type(t: [i16;ITEMS_IN_TYPE]){
    for i in 0..ITEMS_IN_TYPE{
        if t[i] != 0{
            match i {
                1 =>  print!("{}: {} ",Type::Slashing, t[i]),
                2 =>  print!("{}: {} ",Type::Piercing, t[i]),
                3 =>  print!("{}: {} ",Type::Bludgeoning, t[i]),
                4 =>  print!("{}: {} ",Type::Poison, t[i]),
                5 =>  print!("{}: {} ",Type::Acid, t[i]),
                6 =>  print!("{}: {} ",Type::Fire, t[i]),
                7 =>  print!("{}: {} ",Type::Cold, t[i]),
                8 =>  print!("{}: {} ",Type::Radiant, t[i]),
                9 =>  print!("{}: {} ",Type::Necrotic, t[i]),
                10 => print!("{}: {} ",Type::Lightning,t[i]),
                11 => print!("{}: {} ",Type::Thunder, t[i]),
                12 => print!("{}: {} ",Type::Force, t[i]),
                13 => print!("{}: {} ",Type::Psychic, t[i]),
                _ =>  print!("")
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Ignored =>  write!(f,""),
            Type::Slashing => write!(f,"Slashing"),
            Type::Piercing => write!(f,"Piercing"),
            Type::Bludgeoning => write!(f,"Bludgeoning"),
            Type::Poison => write!(f,"Poison"),
            Type::Acid => write!(f,"Acid"),
            Type::Fire => write!(f,"Fire"),
            Type::Cold => write!(f,"Cold"),
            Type::Radiant => write!(f,"Radiant"),
            Type::Necrotic => write!(f,"Necrotic"),
            Type::Lightning => write!(f,"Lightning"),
            Type::Thunder => write!(f,"Thunder"),
            Type::Force => write!(f,"Force"),
            Type::Psychic => write!(f,"Psychic"),
        }
     }
}


impl fmt::Debug for DamageRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        writeln!(f, "Damage Roll:").expect("Nothing");

        display_dmg_type(self.dmg_by_type);

        writeln!(f, "Total: {}, {} ", self.total, if self.modifier > 0 {format!("{}{}","Modifier: +", self.modifier)} else {format!("")})
    }
}


