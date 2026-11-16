use std::fs;
use std::env;

fn main() {
    let os = env::consts::OS;
    println!("[ INFO ] Detectando OS");

    match os {
        "linux" =>  
            println!("Sistema operativo: {}", os),
        "windows" =>
            println!("Sistema operativo: {}", os),
        _ => println!("Sistema operativo desconocido")
    }
}
