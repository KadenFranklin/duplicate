use std::{env, fs};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let vanilla = &args[1] ;
    let chocolate = &args[2];
    if vanilla == &args[1] && chocolate == &args[2] {
        fs::copy(vanilla, chocolate)?;
    } else{println!("Incorrect usage, please try again.") }
    Ok(())
}

