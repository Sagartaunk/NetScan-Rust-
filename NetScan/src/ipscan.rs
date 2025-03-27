use std::sync::{Arc, Mutex};
use crate::{cli , save};
use tokio::net::TcpStream;
use futures;
use tokio::time::{timeout, Duration};

pub async fn run(){
    let i = cli::option_input_ip();
    match i { 
        1 => {
            single_ip_test().await;
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
    futures::future::join_all(tasks).await;
    save::save(open_ports.lock().unwrap().to_vec());
}