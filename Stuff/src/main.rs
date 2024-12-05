fn main() {
    
    println!("Hello, world!");

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple float = {}", tuple.1);

    let array_initialized: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array_initialized[1] = {}", array_initialized[1]);
    
    let array_filled: [u32; 10] = [0xff; 10];
    println!("array_filled[1] = {}", array_filled[1]);
}
