use std::io;
use NetScan::ipscan;
#[tokio::main]
async fn main(){
    println!("Welcome to NetScan");
    println!("Please select one of the following options : ");
    println!("1. Scan IP address for open ports");
    

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input : u8 = input.trim().parse().expect("Please enter a Valid option");
    match input {
        1 => {
            ipscan::run().await;
        }
        _ => {
            println!("Invalid option or under development");
        }
    }
}
