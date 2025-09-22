use std::{env, process::exit};
#[path ="lib.rs"]
mod lib;

use lib::Sock;
fn print_help(){
    println!("Usage: 
        ./scraper.exe target_url -flags(options)    : scraps url

        -analize                     : analizes components of target
        -check                       : checks response code
        -h                           : prints this help message
        warn: this shit sucks can't even scrap a proper url..
        ");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sock = Sock { buffer: [0u8; 4096] };

    if args.len() < 2 {
        print_help();
        exit(0);
    }

    let target = args[1].clone();
    let option = if args.len() > 2 { Some(args[2].as_str()) } else { None };

    match option {
        Some("-analize") => {
            Sock::connect_to_target(&mut sock, target, 1).expect("ERROR");
            Sock::analize_components(&mut sock);
        }
        Some("-check") => {
            Sock::connect_to_target(&mut sock, target, 0).expect("ERROR");
        }
        Some("-h") => {
            print_help();
        }
        Some(_) => {
            println!("Unknown option.");
            print_help();
        }
        None => {
            Sock::connect_to_target(&mut sock, target, 0).expect("ERROR");
        }
    }
}