//understanding tuple in rust

fn main(){
    let tup:(i32, u32, f64) = (-32, 32, 32.0);

    let first_value: i32 = tup.0; 

    let second_value: u32 = tup.1; 

    let third_value: f64 = tup.2; 

    println!("{}", first_value); 
    println!("{}", second_value); 
    println!("{}", third_value); 
}