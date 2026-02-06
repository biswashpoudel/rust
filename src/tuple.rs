//understanding tuple in rust

//tuple in rust lets us to store group of different numeric data types together
fn main(){
    let tup:(i32, u32, f64) = (-32, 32, 32.0);

    let first_value: i32 = tup.0; //accessing the first element in the tuple 

    let second_value: u32 = tup.1; //accessing the second element in the tuple

    let third_value: f64 = tup.2; //accessing the third element in the tuple 

    println!("{}", first_value); //printing the first value: returns -32
    println!("{}", second_value); //printing the second value: returns positive 32
    println!("{}", third_value);  //printing the third value: returns float 32.0 
}