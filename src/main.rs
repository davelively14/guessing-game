// Brings io library into scope. The `std::prelude` only brings a few standard types into scope.
// You can bring additional into scope via the `use` statement.
// NOTE: functions more like Alias. Could have called `std::io::stdin` below.
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // By default, variables are immutable. Adding the `mut` to the variable definition makes it
    // mutable.
    // `String::new()` is a function that returns a new instance of `String` type, which
    // is growable and UTF-8 encoded.
    // ::new is an associated function (aka `static method`)
    let mut guess = String::new();

    // stdin() function returs an instance of the `std::io::Stdin` type.
    // Must pass a mutable string to the read_line function. The `&` indicates we're passing by
    // reference instead of value. Note that by definition, referenced variables are immutable
    // unless otherwise noted, like here.
    // `read_line` takes a String and returns an `io::Result` enum, with variants `Ok` and `Err`.
    // Any instance of the type will respond to the `expect` function. `Err` will return what you
    // pass to `expect`, `Ok` just ignores that and returns the value entered.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
