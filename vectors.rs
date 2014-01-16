fn main() {
    let your_fav_nums = ~[1, 2, 3];
    let my_fav_nums = ~[4, 5, 6];

    let our_fav_nums = your_fav_nums + my_fav_nums;

    println!("The third favorite number is {:d}.", our_fav_nums[2]);
}
