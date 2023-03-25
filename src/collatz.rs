// The Collatz conjecture: https://en.wikipedia.org/wiki/Collatz_conjecture
pub fn collatz(mut num: usize) {
    // The Collatz conjecture only works properly on positive integers; usize only allows positive integers.
    // Therefore, we don't need to handle if a negative number (or float) is plugged in.

    loop {
        println!("{num}");
        // When the integer hits the 4 -> 2 -> 1 range (and it will) we can exit.
        // This also helps to make sure we don't do any unnecessary calculations.
        if num == 1 {
            return ();
        };
        // If num is even, divide by two; else, multiply by three and add one.
        num = if 0 == num % 2 { num / 2 } else { 3 * num + 1 };
    }
}
