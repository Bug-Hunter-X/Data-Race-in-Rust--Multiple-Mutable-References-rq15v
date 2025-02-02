fn main() {
    let mut x = 5;
    { //Creating a scope
        let y = &mut x; 
        *y = 10; // Modifying x through y
    }
    { //creating another scope
        let z = &mut x; //Now it is allowed because the previous mutable refence went out of scope
        *z = 15; // Trying to modify x through z again
    }
    println!("x = {}", x);
}