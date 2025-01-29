fn main() {
    let mut x = 5;
    { // Create a new scope
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modify x through y
    } // Scope ends, y is dropped
    let z = &x; // z is an immutable reference to x
    println!("x = {}", x); // This will print 10
    println!("z = {}", *z); // This will print 10
} 