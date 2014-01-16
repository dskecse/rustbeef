fn print_vec(v: &[int]) {
    for i in v.iter() {
        println(i.to_str())
    }
}

fn print_vec_str(v: &[~str]) {
    for i in v.iter() {
        println((*i).to_str())
    }
}

fn main() {
    let vec = [1, 2, 3];
    print_vec(vec);

    let str_vec = [~"hey", ~"there", ~"yo"];
    print_vec_str(str_vec);
}