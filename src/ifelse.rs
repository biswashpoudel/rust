fn calculate_even(number: i32)-> bool{
    if number % 2 == 0 {
        return true; 
    } else{
        return false; 
    }
}
fn main(){
    let output = calculate_even(21); 
    println!("{}", output); //returns false as 21 is odd
}