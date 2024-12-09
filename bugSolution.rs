fn main() {
    let mut x = 5;
    {
        let y = &mut x; 
        *y = 10; 
    } 
    let z = &x; 
    println!("The value of x is {} and z is {}", x, *z);
}

//Alternative solution:
fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    println!("The value of x is {}", x);
}