//writing a function to read a file using fs in rust 

use std::fs; 
fn main (){
    let contents = fs::read_to_string("file.txt"); 
    match contents{
        Ok(contents)=> println!("{}", contents),
        Err(err)=> print!("Err reading this file"), 
    }
}