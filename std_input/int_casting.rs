use std::io::buffered::BufferedReader;
use std::io;

fn main() {
    println("INPUT:");
    let mut reader = BufferedReader::new(io::stdin());

    let input = reader.read_line().unwrap_or(~"nothing");
    let num = from_str::<int>(input.slice_to(input.len() - 1));
    println("YOU TYPED:");
    println(num.to_str());
}
