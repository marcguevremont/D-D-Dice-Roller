extern crate rand;
use std::fmt;
use rand::distributions::{Distribution, Uniform};

pub enum Type {
    Normal,
    Advantage,
    Disadvantage
}

pub enum Outcome{
    Hit,
    Fail,
    Critical,
    Fumble
}

pub struct Attack {
    Type    : Type,
    hits    : u16,
    crits   : u16,
    fails   : u16,
    fumbles : u16,
    modif   : i16,
    crit_on : i16,
    stop_on_fumble: bool,
    ac      : i16,
    history : Vec<(i16,  Outcome)>,
}

impl Attack {
    pub fn new(a:Type ,time :i16, ac : i16, modif:i16, crit_on: i16, stop_on_fumble: bool ) -> Attack{
    
        let mut hits : u16 = 0;
        let mut crits : u16 = 0;
        let mut fumbles: u16 =0;
        let mut misses : u16 =0;
        let mut hist : Vec<(i16,  Outcome)> = vec![];
        print!("throw: ");
        for _ in 0..time {
            let throw = roll_dice(&a);
            let att_roll = throw as i16 + modif;
            if att_roll >= ac {
                //hit
                if throw as i16 >= crit_on {
                    //critical hit
                    crits += 1;
                    hist.push((att_roll,  Outcome::Critical));
                }
                else {
                    hist.push((att_roll,  Outcome::Hit));
                    hits += 1;
                }
            }
            else
            {
                //miss
                if throw == 1 {
                    //critical fail
                    hist.push((throw as i16, Outcome::Fumble));
                    fumbles += 1;
                    if stop_on_fumble {
                        break;
                    }
                }
                else
                {
                    hist.push((att_roll, Outcome::Fail));
                    misses += 1;
                }
            }
        }
        println!("\n");
        Attack{
            Type : a,
            ac : ac,
            crit_on : crit_on,
            stop_on_fumble: stop_on_fumble,
            hits  : hits,
            crits : crits,
            fails : misses,
            fumbles : fumbles,
            modif : modif,
            history : hist
        }
    }
    pub fn get_hits(&self) -> u16 {
        self.hits
    }
    pub fn get_crits(&self) -> u16 {
        self.crits
    }
    pub fn get_fails(&self) -> u16 {
        self.fails
    }
    pub fn get_fumbles(&self) -> u16 {
        self.fumbles
    }
    pub fn get_type(&self) -> &Type {
        &self.Type
    }
    pub fn get_modif(&self) -> i16 {
        self.modif
    }
    pub fn get_ac(&self) -> i16 {
        self.ac
    }
    pub fn get_crit_on(&self) -> i16 {
        self.crit_on
    }
    pub fn get_stop_on_fumble(&self) -> bool {
        self.stop_on_fumble
    }

    pub fn get_history(&self) -> &Vec<(i16,  Outcome)> {
        &self.history 
    }

}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Normal => write!(f, "Normal"),
            Type::Advantage => write!(f, "Advantage"),
            Type::Disadvantage => write!(f, "Disadvantage")
        }
     }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Outcome::Hit => write!(f, "Hit"),
            Outcome::Fail => write!(f, "Fail"),
            Outcome::Critical => write!(f, "Critical"),
            Outcome::Fumble => write!(f, "Fumble"),
        }
     }
}

impl fmt::Debug for Attack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        for i in self.get_history().iter(){
            writeln!(f, "{:<2} {}", i.0, i.1).expect("could not read the tuple");
        }

        writeln!(f, "Attack Type: {}, mod: {}, ac: {}, crit on: {}, stop on fumble: {}", 
        self.get_type(),
        if self.get_modif() > 0 {format!("{}{}", "+", self.get_modif())} else {format!("{}",self.get_modif())},
        self.get_ac(),
        self.get_crit_on(),
        self.get_stop_on_fumble()
        ).expect("unreadable attribute in Attack");

        writeln!(f, "Hits: {}, Crits: {}, Fails: {}, Fumbles: {}",  
        self.get_hits(), 
        self.get_crits(),
        self.get_fails(),
        self.get_fumbles(),
        )
    }
}

fn roll_dice(a : &Type) -> u8 {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..20+1);
    
    match a {
        Type::Normal => {
            let throw = die.sample(&mut rng);
            print!("{} ",throw);
            throw 
        },
        Type::Advantage => {
            let throw1 = die.sample(&mut rng);
            let throw2 = die.sample(&mut rng);
            print!("{}-{} ",throw1, throw2 );
            if throw1  > throw2 {
                throw1
            }
            else{
                throw2
            }
        },
        Type::Disadvantage => {
            let throw1 = die.sample(&mut rng);
            let throw2 = die.sample(&mut rng);
            print!("{}-{} ",throw1, throw2 );
            if throw1 < throw2 {
                throw1
            }
            else{
                throw2
            }
        }
    }
}