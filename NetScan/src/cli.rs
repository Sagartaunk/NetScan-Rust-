use std::io;


pub fn option_input_ip() -> u8 {
    println!("Please select one of the following options :");
    println!("1. Scan a single IP address for open ports");
    println!("2. Scan a range of IP addresses");
    println!("3. Scan a subnet for open ports");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input : u8 = input.trim().parse().expect("Please enter a valid input");
    input
}


pub fn single_ip() -> (String , i64 , i64){
    let mut ip = String::new();
    let mut s_port = String::new();
    let mut e_port = String::new();
    println!("Please enter the ip address you want to scan : ");
    io::stdin().read_line(&mut ip).expect("Failed to read line");
    println!("Please enter the starting port : ");
    io::stdin().read_line(&mut s_port).expect("Failed to read line");
    println!("Please enter the ending port : ");
    io::stdin().read_line(&mut e_port).expect("Failed to read line");
    let s_port : i64 = s_port.trim().parse().expect("Please enter a valid port number");
    let e_port : i64 = e_port.trim().parse().expect("Please enter a valid port number");
    (ip.trim().to_string() , s_port , e_port)
}

pub fn ip_range() -> (String , String){
    let mut s_ip = String::new();
    let mut e_ip = String::new();
    println!("Please enter the starting ip address : ");
    io::stdin().read_line(&mut s_ip).expect("Failed to read line");
    println!("Please enter the ending ip address : ");
    io::stdin().read_line(&mut e_ip).expect("Failed to read line");
    (s_ip.trim().to_string() , e_ip.trim().to_string())
}

pub fn domain() -> (String , u8){
    println!("Please enter the url you want to crawl : ");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read url");
    let url = url.trim().to_string();
    println!("Please enter the max depth you want to crawl : ");
    let mut max_depth = String::new();
    io::stdin().read_line(&mut max_depth).expect("Failed to read max depth");
    let max_depth : u8 = max_depth.trim().parse().expect("Please enter a valid depth");
    (url,max_depth)
    
}