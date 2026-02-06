fn main(){

    //tuple lets us to store the numeric data types together
    //i32 stores the signed integers
    // u32 stores the unsigned int which are always +ve
    // f64 stores the floating points
    let tup: (u32, i32, f64) = (20, -98, 98.76); 

    let first_value: u32 = tup.0; 
    let second_value: i32 = tup.1; 
    let third_value: f64 = tup.2; 

    println!("{}", first_value); 
    println!("{}", second_value); 
    println!("{}", third_value); 
}