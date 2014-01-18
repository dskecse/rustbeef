use std::rand::Rng;

fn main() {
    let n = std::rand::rng().gen_range(1, 101);
    println(n.to_str());
}
