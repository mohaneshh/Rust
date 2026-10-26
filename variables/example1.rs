// Binding and Mutability:

// 1) A variable can be used only if it has been initialized. 

fn main() {
    let x: i32 = 5; // If Uninitialized and used, Error.
    let _y: i32; // If Uninitialized but also not used, Just a warning. Use a prefix underscore to fix the warning. 

    assert_eq!(x, 5); // Checks if both value or equal or not. 
    print!("Success!")
}