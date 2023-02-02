use std::fs::File;
use std::io::{self, stdin, stdout, BufRead, BufWriter, Read, Write};
use std::net::SocketAddr;
use std::{env, error};

fn main() {
    let mut file_name = String::new();
    println!("Enter input file: ");
    let _ = stdout().flush();

    stdin()
        .read_line(&mut file_name)
        .expect("Did not enter a vaild string\n");
    //stdin().read_buf(&mut file_name);
    file_name = file_name.trim().to_string();

    //s = "./test.txt".to_string();

    // Breaks if bad input
    // Bad bad bad
    //let input: SocketAddr = fs::read_to_string(s);

    let input_file_result = File::open(file_name.to_string());
    let input_file = match input_file_result {
        Ok(file) => file,
        Err(error) => panic!(
            "At the disco: {:?} . Sadge. Filename was: {} ---",
            error, file_name
        ),
    };

    let mut elf = Vec::new();
    let mut CurrentElf: i128 = 0;

    //let x;
    /*  for i in 0..10 {
        elf.push(i);
    }

    for _ in 0..11 {
        let x = elf.pop();
        match x {
            Some(z) => println!("x = {} \n", z),
            None => println!("error"),
        }
    }*/

    let test_line = io::BufReader::new(input_file).lines();

    for line in test_line {
        //Fine Error in file
        let current_item_string = match line {
            Ok(line_string) => line_string,
            Err(_error) => panic!("error in reading file"),
        };
        // Detect if its a new line or not
        let current_item: i128 = match current_item_string.parse() {
            Ok(item) => item,
            Err(_error) => 0,
        };
        if current_item == 0 {
            elf.push(CurrentElf);
            CurrentElf = 0;
        }
        else{
            CurrentElf = CurrentElf + current_item;
        }
    }
    /*for i in elf{
        println!("elf stuff {}", i);
    }*/
    elf.sort();
    println!("top 3 total {}" , elf[elf.len()-1] + elf[elf.len()-2] + elf[elf.len()-3]);

}
