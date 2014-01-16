enum Monster {
    ScubaArgentine(int, int, int, int),
    IndustrialRaverMonkey(int, int, int, int)
}

impl Monster {
    fn attack(&self) {
        match *self {
            ScubaArgentine(_, _, _, w) => println!("The monster attacks for {:d} damage.", w),
            IndustrialRaverMonkey(_, _, _, w) => println!("The monster attacks for {:d} damage.", w)
        }
    }
}

fn main() {
    let irm = IndustrialRaverMonkey(46, 35, 91, 2);

    irm.attack();
}
