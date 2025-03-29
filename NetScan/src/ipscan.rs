use std::sync::{Arc, Mutex};
use crate::{cli, save};
use tokio::net::TcpStream;
use futures::future;
use tokio::time::{timeout, Duration};


pub async fn run(){
    let i = cli::option_input_ip();
    match i { 
        1 => {
            single_ip_test().await;
        }
        2 => {
            ip_range_test().await;
        }
        3=> {
            local_net().await;
        }
        _ => {
            println!("Invalid option or under development");
        }
    }
}

pub async fn single_ip_test(){
    let (ip , s_port , e_port) = cli::single_ip();
    println!("Scanning tp ip address {} , from port {} to port {}" , &ip , &s_port , &e_port);
    let ports : Vec<i64> = (s_port..=e_port).collect();
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let tasks: Vec<_> = ports
        .into_iter()
        .map(|port| {
            let ip = ip.clone();
            let open_ports = Arc::clone(&open_ports);
            tokio::spawn(async move {
                let address = format!("{}:{}", ip, port);
                let result = timeout(Duration::from_secs(3), TcpStream::connect(address.clone())).await;
                match result {
                    Ok(_) => {
                        println!("Port {} is open", port);
                        let mut open_ports = open_ports.lock().unwrap();
                        open_ports.push(address);
                    }
                    Err(_) => {}
                }
            })
        })
        .collect();
    future::join_all(tasks).await;
    save::save(open_ports.lock().unwrap().to_vec());
}

pub async fn ip_range_test(){
    let (start , end) = cli::ip_range();
    let ip_range = ip_range(start , end);
    let open_ip = Arc::new(Mutex::new(Vec::new()));
    let tasks : Vec<_> = ip_range
        .into_iter()
        .map(|ip| {
            let open_ip = Arc::clone(&open_ip);
            tokio::spawn(async move {
                let result = timeout(Duration::from_secs(3), TcpStream::connect(ip.clone())).await;
                match result{
                    Ok(_) => {
                        println!("Ip {} is open" , ip);
                        let mut  open_ip = open_ip.lock().unwrap();
                        open_ip.push(ip);
                    }
                    Err(_) => {}
                }
            })
        })
        .collect();
    future::join_all(tasks).await;
    save::save(open_ip.lock().unwrap().to_vec());

}

pub fn ip_range(start : String , end : String) -> Vec<String>{
    let start = start.split(".").collect::<Vec<&str>>();
    let end = end.split(".").collect::<Vec<&str>>();
    let mut ip_range = Vec::new();
    let start : [u8 ; 5] = [start[0].parse().unwrap() , start[1].parse().unwrap() , start[2].parse().unwrap() , start[3].parse().unwrap() , 80];
    let end : [u8 ; 5] = [end[0].parse().unwrap() , end[1].parse().unwrap() , end[2].parse().unwrap() , end[3].parse().unwrap() , 80];
    for i in start[0]..=end[0]{
        for j in start[1]..=end[1]{
            for k in start[2]..=end[2]{
                for l in start[3]..=end[3]{
                    ip_range.push([i , j , k , l , 80]);
                }
            }
        }
    }
    ip_range.into_iter().map(|ip| {
        format!("{}.{}.{}.{}:{}", ip[0] , ip[1] , ip[2] , ip[3] , ip[4])
    }).collect()
}

pub async fn local_net(){
    println!("Starting the scan");
    let vec = Arc::new(Mutex::new(Vec::new()));
    let tasks : Vec<_> = (1..=255).into_iter().map(|i| {
        let vec = Arc::clone(&vec);
        tokio::spawn(async move {
            let ip = format!("192.168.1.{}:80" , i);
            let result = timeout(Duration::from_secs(3), TcpStream::connect(ip.clone())).await;
            match result{
                Ok(_) => {
                    println!("Ip {} is open" , ip);
                    let mut vec = vec.lock().unwrap();
                    vec.push(ip);
                }
                Err(_) => {}
            }
        })
    }).collect();
    future::join_all(tasks).await;
    save::save(vec.lock().unwrap().to_vec());
}