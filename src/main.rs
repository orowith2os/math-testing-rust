mod collatz;
mod fibonacci;

fn main() {
    println!("Hello, world!");
    // TODO: We should ask for user input, and take CLI arguments to see what math funtions the user wants to do.

    collatz::collatz(7323);
    fibonacci::fibonacci(84);
}
