fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; // Modifying x through y
    let z = &mut x; // This is where the error might occur
    *z = 15; // Trying to modify x through z again
    println!("x = {}", x);
}