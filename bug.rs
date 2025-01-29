fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y = 10; // Modify x through y
    let z = &x; // z is an immutable reference to x
    println!("x = {}", x); // This will print 10
    println!("z = {}", *z); // This will print 10
    // The following line will cause a compiler error
    // *y = 10;
}