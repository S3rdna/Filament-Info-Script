enum MaterialType {
    PLA,
    PETG,
    ABS,
}

enum MaterialModifier {
    BASIC,
    TRANSLUCENT,
    MATTE,
    METAL,
    GLOW,
    MARBLE,
    SILK,
    GRADIENT,
    DUAL,
}

struct Filament {
    material: MaterialType,
    modifier: MaterialModifier,
    price: f32,
}

fn display_gui() {
    println!("Select Option");
    println!("1. Check Filament");
    println!("2. Add Filament");
}

#[derive(Debug)]
enum UserInputError {
    NoInputError,
    WrongSizeInputError,
    UserInputParseError,
}

fn get_user_input() -> Result<u8, UserInputError> {
    use std::io::{self};

    let mut input = String::new(); //create buffer to hold input
    let _ = io::stdin().read_line(&mut input); // create stdin struct
    input = input.trim().to_string(); //trim input

    match input.len() {
        0 => Err(UserInputError::NoInputError),
        1 => match input.parse::<u8>() {
            Err(_) => Err(UserInputError::UserInputParseError),
            Ok(k) => Ok(k),
        },
        _ => Err(UserInputError::WrongSizeInputError),
    }
}

fn main() {
    loop {
        display_gui(); //ui
        match get_user_input() {
            Ok(k) => println!("{k}"),
            Err(e) => println!("{:?}", e),
        }
        //save data in file
        //calculate remaining filament
    }
}
