use std::fs;
pub fn save(ip : Vec<String>){
    match fs::File::open("ip.txt"){
        Ok(_) =>{
            for i in ip.iter(){
                fs::write("ip.txt" , &i).expect("Failed to write");
            }
            

        },
        Err(_) => {
            fs::File::create("ip.txt").expect("Failed to create file");
            for i in ip.iter(){
                fs::write("ip.txt" , &i).expect("Failed to write");
            }
        }
    }
}