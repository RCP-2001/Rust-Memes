use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Write, self, BufRead, BufWriter, Read};
use std::net::SocketAddr;



fn main() {
    let mut file_name = String::new();
    println!("Enter input file: ");
    let _ = stdout().flush();

    stdin().read_line(&mut file_name).expect("Did not enter a vaild string\n");
    //stdin().read_buf(&mut file_name);
    file_name = file_name.trim().to_string();
    
    //s = "./test.txt".to_string();

    // Breaks if bad input
    // Bad bad bad
    //let input: SocketAddr = fs::read_to_string(s);

    let input_file_result = File::open(file_name.to_string());
    let input_file = match input_file_result {
        Ok(file) => file,
        Err(error) => panic!("At the disco: {:?} . Sadge. Filename was: {} ---", error, file_name),
    };
    
    

    let test_line = io::BufReader::new(input_file).lines();

    for line in test_line{
        println!("Input: {} ", line.unwrap() );
    }
}

