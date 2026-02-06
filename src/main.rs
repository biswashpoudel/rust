// writing the tuple syntax in rust 

//entry point of the rust function 

//tuple lets us to store a group of different numeric data types together inside 
//small brackets 
fn main(){ 
//declaring a tuple variable with three different data types

let tup: (i32, u32, f64) = (-27, 27, 27.27); 

//accessing the tuple elements like accessing the js arrays elements

let first_value: i32 = tup.0;

let second_value: u32 = tup.1; 

let third_value: f64 = tup.2; 

//printing the values into the command line

println!("{}", first_value); //should print the negative 27

println!("{}", second_value); //ashould return the positive 27

println!("{}", third_value); //should return the floating value 27.27

}