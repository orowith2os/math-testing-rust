// The Fibonacci sequence: https://en.m.wikipedia.org/wiki/Fibonacci_sequence

pub fn fibonacci(mut num: usize) {
    // Our current place in the sequence of numbers
    let mut spot: usize = 1;
    let mut sum = 0;
    // The current number's spot minus one
    let mut before: usize = 0;
    // The current number (in spot)
    let mut current: usize = 1;

    for _i in 0..num {
        println!("{spot}: {sum}");
        sum = before + current;
        before = current;
        current = sum;
        spot += 1;
    }
}
