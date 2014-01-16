fn message(i: int) {
    match i {                                       // sort of a case statement
        1 => println("ONE!"),
        2 => println("Two is a prime."),
        3 => println("THREE!"),
        _ => println("no idea what that is, boss")  // default case
    }
}

fn main() {
    message(1);
    message(2);
    message(3);
}
