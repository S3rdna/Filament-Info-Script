use std::io::Read;

use chrono::Local;

#[derive(Debug)]
enum MaterialType {
    PLA,
    PETG,
    ABS,
}

impl MaterialType {
    pub fn into_iter() -> core::array::IntoIter<MaterialType, 3> {
        [MaterialType::PLA, MaterialType::PETG, MaterialType::ABS].into_iter()
    }

    pub fn into_string_iter() -> core::array::IntoIter<String, 3> {
        ["PLA".to_string(), "PETG".to_string(), "ABS".to_string()].into_iter()
    }
}

#[derive(Debug)]
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

impl MaterialModifier {
    pub fn into_iter() -> core::array::IntoIter<MaterialModifier, 9> {
        [
            MaterialModifier::BASIC,
            MaterialModifier::TRANSLUCENT,
            MaterialModifier::MATTE,
            MaterialModifier::METAL,
            MaterialModifier::GLOW,
            MaterialModifier::MARBLE,
            MaterialModifier::SILK,
            MaterialModifier::GRADIENT,
            MaterialModifier::DUAL,
        ]
        .into_iter()
    }

    pub fn into_string_iter() -> core::array::IntoIter<String, 9> {
        [
            "BASIC".to_string(),
            "TRANSLUCENT".to_string(),
            "MATTE".to_string(),
            "METAL".to_string(),
            "GLOW".to_string(),
            "MARBLE".to_string(),
            "SILK".to_string(),
            "GRADIENT".to_string(),
            "DUAL".to_string(),
        ]
        .into_iter()
    }
}

#[derive(Debug)]
struct Filament {
    material: MaterialType,
    modifier: MaterialModifier,
    price: f32,
    buy_date: String,
}

fn display_gui() {
    println!("Select Option");
    println!("1. Check Filament");
    println!("2. Add Filament");
    println!("3. New Print");
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

fn filament_builder() -> Filament {
    use std::io::{self};
    //choose material
    let new_mat: MaterialType;
    let new_mod: MaterialModifier;
    let new_price: f32;
    let new_date: String;

    println!("choose material");
    println!("====================");

    let mut count = 0;
    for mat in MaterialType::into_iter() { // loop thru mat types 
        println!("{count}. {:?}",mat);
        count += 1;
    }

    let mut fil_choice = [0;2];
    let _ = io::stdin().read(&mut fil_choice);

    println!("mat:{:?}",fil_choice);

    //choose modifier
    println!("choose mod");
    println!("====================");

    let mut count = 0;
    for mat_mod in MaterialModifier::into_iter() { // loop thru mod types 
        println!("{count}. {:?}",mat_mod);
        count += 1;
    }

    fil_choice = [0;2];
    let _ = io::stdin().read_to_string(&mut fil_choice);

    println!("mod:{:?}",fil_choice);

    // get price
    print!("Price:");
    let mut price = String::new(); //create buffer to hold input
    let _ = io::stdin().read_line(&mut price); // create stdin struct
    match price.trim().to_string().parse::<f32>() {
        Ok(k) => new_price = k,
        Err(_) => new_price = 20.08,
    };
    print!("\n");

    // get date
    let now = Local::now();
    let new_date =  now.date_naive().to_string();
    println!("{:?}",new_date);

    // return filament
    Filament {
        material: new_mat,
        modifier: new_mod,
        price: new_price,
        buy_date: new_date,
    }
}

fn add_filament() {
    use csv::WriterBuilder;
    use std::fs::File;

    let new_filament = filament_builder();
    println!("filament: {:?}",new_filament);

    let file = File::open("filamentDB.csv").unwrap();
    let mut wtr = WriterBuilder::new().from_writer(file);
    wtr.write_record(&["a", "betoasijadso", "c"]).unwrap();
    println!("Done Writing");
}

fn main() {
    loop {
        display_gui(); //ui
        match get_user_input() {
            Ok(k) => match k {
                1 => println!("{k}"), // check filament
                2 => add_filament(),  // add filament
                3 => println!("{k}"), // new print
                _ => println!("choose another"),
            },
            Err(e) => println!("{:?}", e),
        }
        //save data in file
        //calculate remaining filament
    }
}
