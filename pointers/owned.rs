fn main() {
    let x: ~int = ~10;
    let y = x.clone();
    println((*x).to_str());
}
