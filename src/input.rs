
use std::io;
pub fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

pub fn get_char_input_with_default(prompt: &str, choice: &Vec<char>, default : char) -> char{
    let input = get_input(prompt);
    
    if input.trim().is_empty(){
        default
    }
    else if input.trim().len() == 1 {
        let c : char = input.chars().next().unwrap();
        let mut found = false;
        for &i in choice.iter(){
            if i == c {
                found = true;
            }
        }
        if found {
            c
        }
        else
        {
            get_char_input_with_default(prompt,choice, default)
        }
    }
    else
    {
        get_char_input_with_default(prompt,choice, default)
    }

}

pub fn get_i16_input(prompt: &str) -> i16{
    let input = get_input(prompt);

    let test = input.to_string().trim().parse::<i16>();
    match test {
        Ok(ok) => ok,
        Err(_e) => get_i16_input(prompt), 
    } 
}

pub fn get_i16_input_with_default(prompt: &str, default: i16) -> i16{
    let input = get_input(prompt);
    
    if input.trim().is_empty(){
        default
    }
    else
    {
        let test = input.to_string().trim().parse::<i16>();
        match test {
            Ok(ok) => ok,
            Err(_e) => get_i16_input_with_default(prompt, default), 
        } 
    }
    
}
