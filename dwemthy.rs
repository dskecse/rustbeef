trait Monster {         // let all monsters to implement `attack`
    fn attack(&self);
}

struct IndustrialRavenMonkey {
    strength: int
}

impl Monster for IndustrialRavenMonkey {
    fn attack(&self) {
        println!("The monkey attacks for {:d}.", self.strength)
    }
}

impl Monster for int {  // write an implementation for absolutely anything
    fn attack(&self) {
        println!("The int attacks for {:d}.", *self)
    }
}

fn main() {
    let monkey = IndustrialRavenMonkey { strength: 35 };
    monkey.attack();

    let i = 10;
    i.attack();
}
