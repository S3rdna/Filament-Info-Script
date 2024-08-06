use std::{borrow::Borrow, fmt::Display};

use serde::Serialize;

// *** custom types
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
enum MaterialType {
    PLA,
    PETG,
    ABS,
}

impl Display for MaterialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl MaterialType {
    pub fn into_iter() -> core::array::IntoIter<MaterialType, 3> {
        [MaterialType::PLA, MaterialType::PETG, MaterialType::ABS].into_iter()
    }

    pub fn into_string_iter() -> core::array::IntoIter<String, 3> {
        ["PLA".to_string(), "PETG".to_string(), "ABS".to_string()].into_iter()
    }

    pub fn choose(c: u8) -> Result<MaterialType, UserInputError> {
        match c {
            0 => Ok(Self::PLA),
            1 => Ok(Self::PETG),
            2 => Ok(Self::ABS),
            _ => Err(UserInputError::OutOfBoundsInputError),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
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

impl Display for MaterialModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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

    pub fn choose(c: u8) -> Result<MaterialModifier, UserInputError> {
        match c {
            0 => Ok(Self::BASIC),
            1 => Ok(Self::TRANSLUCENT),
            2 => Ok(Self::MATTE),
            3 => Ok(Self::METAL),
            4 => Ok(Self::GLOW),
            5 => Ok(Self::MARBLE),
            6 => Ok(Self::SILK),
            7 => Ok(Self::GRADIENT),
            8 => Ok(Self::DUAL),
            _ => Err(UserInputError::OutOfBoundsInputError),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct Filament {
    material: MaterialType,
    modifier: MaterialModifier,
    color: String,
    price: f32,
    buy_date: String,
    weight: f32,
}

impl Filament {
    fn use_filament(&mut self, psize: f32) {
        self.weight -= psize;
    }
}

impl Display for Filament {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}/{}:[Color:{},Reserve:{}]",
            self.material, self.modifier, self.color, self.weight
        )
    }
}

#[derive(Debug)]
enum UserInputError {
    NoInputError,
    WrongSizeInputError,
    UserInputParseError,
    OutOfBoundsInputError,
}

// *** GEN Functions

fn display_gui() {
    println!("\nSelect Option");
    println!("====================");
    println!("1. Check Filament");
    println!("2. Add Filament");
    println!("3. New Print");
    println!("4. Edit DB");
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

// ** option 2
fn filament_builder() -> Filament {
    use chrono::Local;
    use std::io::{self, Read};

    //init
    let new_mat: MaterialType;
    let new_mod: MaterialModifier;
    let new_price: f32;
    let new_date: String;
    let mut new_color: String;

    println!("\nChoose Material");
    println!("====================");

    let mut count = 0;
    for mat in MaterialType::into_iter() {
        // loop thru mat types
        println!("{count}. {}", mat);
        count += 1;
    }

    let mut buffer = [0; 3];
    let _ = io::stdin().read(&mut buffer);
    new_mat = MaterialType::choose(buffer[0] - 48).unwrap();

    //choose modifier
    println!("\nChoose Mod");
    println!("====================");

    let mut count = 0;
    for mat_mod in MaterialModifier::into_iter() {
        // loop thru mod types
        println!("{count}. {}", mat_mod);
        count += 1;
    }

    buffer = [0; 3];
    let _ = io::stdin().read(&mut buffer);

    new_mod = MaterialModifier::choose(buffer[0] - 48).unwrap();

    // get color
    println!("\nEnter Color");
    println!("====================");
    new_color = String::new(); //create buffer to hold input
    let _ = io::stdin().read_line(&mut new_color); // create stdin struct

    // get price
    println!("\nEnter Price");
    println!("====================");

    let mut price = String::new(); //create buffer to hold input
    let _ = io::stdin().read_line(&mut price); // create stdin struct
    match price.trim().to_string().parse::<f32>()  /*handle float parse*/ {
        Ok(k) => new_price = k,
        Err(_) => new_price = 20.08,
    };

    // get date
    let now = Local::now();
    new_date = now.date_naive().to_string();

    // return filament
    Filament {
        material: new_mat,
        modifier: new_mod,
        color: new_color.trim().to_string(),
        price: new_price,
        buy_date: new_date,
        weight: 100.0,
    }
}

fn add_filament(f: Filament, is_append: bool) {
    use csv::WriterBuilder;
    use std::fs::{metadata, OpenOptions};

    let file_path = "filamentDB.csv";
    let file_exists = match metadata(file_path) {
        Ok(metadata) => metadata.len() > 0,
        Err(_) => false,
    };
    let new_filament = f;
    println!("filament: {}", new_filament);

    match is_append {
        true => {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(file_path)
                .unwrap();

            let mut wtr = WriterBuilder::new()
                .has_headers(!file_exists)
                .from_writer(file);

            wtr.serialize(new_filament).unwrap();
            wtr.flush().unwrap();
            println!("Done Writing");
        }
        false => {
            let mut wtr = WriterBuilder::new()
                .has_headers(true)
                .from_path(file_path)
                .unwrap();
            wtr.serialize(new_filament).unwrap();
            wtr.flush().unwrap();
            println!("Done Writing");
        }
    }
}

// ** option 1
fn check_db() -> Vec<Filament> {
    use csv::ReaderBuilder;

    let mut ret: Vec<Filament> = vec![];

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("filamentDB.csv")
        .unwrap();

    let iter = rdr.deserialize();
    for record in iter {
        let n: Filament = record.unwrap();
        ret.push(n);
    }
    ret
}

//** option 3
fn new_print() -> Result<(), UserInputError> {
    use std::io;
    //get amount being used in new print

    println!("how big of a print?");
    println!("====================");
    let mut choice = String::new();
    let _ = io::stdin().read_line(&mut choice);
    match choice.trim().to_string().parse::<f32>() {
        Ok(k) => {
            let material_used = k;

            //get data from db
            let mut filaments = check_db();

            //diaplay filaments
            println!("Choose which filament to use");
            println!("====================");
            let mut ind = 0;
            for fil in filaments.clone().into_iter() {
                if fil.weight >= material_used {
                    println!("{} ==> {}", ind, fil);
                }
                ind += 1;
            }

            //create buffer and get user input
            choice = String::new();
            let _ = io::stdin().read_line(&mut choice);

            let _ = match choice.trim().to_string().parse::<usize>() {
                Ok(k) => {
                    filaments
                        .as_mut_slice()
                        .into_iter()
                        .nth(k)
                        .unwrap()
                        .use_filament(material_used);

                    // add first and remove first from vec
                    add_filament(filaments.get(0).unwrap().to_owned(), false);
                    filaments.remove(0);
                    for ele in filaments.into_iter() {
                        add_filament(ele, true);
                    }

                    Ok(())
                }
                Err(_) => Err(UserInputError::UserInputParseError),
            };

            Ok(())
        }
        Err(_) => Err(UserInputError::UserInputParseError),
    }
    //user choose which filament
    //choose amount
}

// *** main
fn main() {
    loop {
        display_gui(); //ui
        match get_user_input() {
            Ok(k) => match k {
                1 => {
                    for fil in check_db().into_iter() {
                        println!("{}", fil);
                    }
                    ()
                } // check filament
                2 => add_filament(filament_builder(), true), // add filament
                3 => new_print().unwrap(),                   // new print
                4 => println!("test:{k}"),                   // edit db
                _ => println!("choose another"),
            },
            Err(e) => println!("{:?}", e),
        }
        //calculate remaining filament
    }
}
