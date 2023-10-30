#![allow(warnings)]
#![allow(clippy::all)]
use std::env;
use IDP2_yuhan_rust::{create, delete, load, read, update};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "Please specify the operation to perform. Usage: {} [action]",
            args[0]
        );
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "load" => {
            load("diabetes.csv").expect("Error during loading");
            println!("Load Success!");
        }
        "create" => {
            if args.len() <= 2 {
                // let query_str = &args[2];
                if let Err(err) = create(4, 94, 78, 31, 85, 33.1, 0.446, 22, 1) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Create Success!");
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        "read" => {
            if args.len() <= 2 {
                // let query_str = &args[2];
                if let Err(err) = read() {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Read Success!");
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        "update" => {
            if args.len() <= 2 {
                // let query_str = &args[2];
                if let Err(err) = update(1, 4, 94, 78, 31, 85, 33.1, 0.446, 22, 1) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Update Success!");
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        "delete" => {
            if args.len() <= 2 {
                // let query_str = &args[2];
                if let Err(err) = delete(10) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Delete Success!");
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        _ => {
            println!("Please only use one of the following commands: 'load', 'create', 'delete', 'update' or 'read' command.");
        }
    }
}
