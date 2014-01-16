struct Monster {
    health: int,
    attack: int
}

impl Monster {
    // constructor - associated function
    fn new(health: int, attack: int) -> Monster {
        Monster { health: health, attack: attack }
    }

    // associated function as well
    fn count() {
        println("There are a bunch of monsters out tonight.");
    }

    // method
    fn attack(&self) {
        println!("The monster attacks for {:d} damage.", self.attack);
    }
}

fn main() {
    let m = Monster::new(20, 40);

    println!("{:?}", m);                  // take a struct using :? format specifier
    println!("health: {:d}", m.health);
    m.attack();
    Monster::count();
}
