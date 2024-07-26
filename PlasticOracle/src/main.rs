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
}

fn display_gui() {
    use std::io::{self, Write};

    io::stdout().write(b"this").expect("test");
    println!("Select Option");
    println!("1. Check Filament");
    println!("2. Add Filament");
}

fn main() {
    //gui
    display_gui();
    //save data in file
    //calculate remaining filament
}
