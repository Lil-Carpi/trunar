use std::env;
use std::io;
#[derive(PartialEq)]
enum OSs {
    Linux,
    Windows,
    Unknown
}
fn main() {
    let os = env::consts::OS;
    println!("[ INFO ] Detecting OS");
    let current_status = match os {
        "linux"   => OSs::Linux,
        "windows" => OSs::Windows,
        _         => OSs::Unknown,
    };
    match current_status {
        OSs::Linux   => println!("[  OK  ] Operating system: {}", os),
        OSs::Windows => println!("[  OK  ] Operating system: {}", os),
        OSs::Unknown => println!("[  OK  ] Unknown operating system {}", os),
    }
    if current_status == OSs::Unknown {
        println!("Do you want to specify your operating system?");
        let mut os_specify = String::new();
        io::stdin()
            .read_line(&mut os_specify)
            .expect("[ Error ] Error reading line"); 
        println!("[ INFO ] You specified: {}", os_specify.trim());
    }
}
