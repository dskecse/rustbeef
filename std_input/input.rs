use std::io::buffered::BufferedReader;
use std::io;

fn main() {
    println("INPUT:");
    let mut reader = BufferedReader::new(io::stdin());

    let input = reader.read_line().unwrap_or(~"nothing");
    println("YOU TYPED:");
    println(input);
}
