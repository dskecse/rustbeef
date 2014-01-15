fn main() {
    for _ in range(0, 10) {
        do spawn {
            let greeting_message = "Hello?";
            println(greeting_message);
        }
    }
}
