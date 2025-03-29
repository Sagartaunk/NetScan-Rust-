use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
pub fn save(ip : Vec<String>){
    let path  = "ip.txt" ;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&path)
        .unwrap();
    for i in ip{
        writeln!(file , "{}" , i).unwrap();
    }
}
pub fn save_domains(domains : Vec<String>) {
    let path  = "crawler.txt" ;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&path)
        .unwrap();
    for i in domains{
        writeln!(file , "{}" , i).unwrap();
    }
}