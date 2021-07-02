use art_benchmarks::mix;
use art_benchmarks::PrimaryColor;

fn main() {
    println!("Hello, world!");
    let c1 = PrimaryColor::Red;
    let c2 = PrimaryColor::Blue;

    println!("{:?}", mix(c1, c2));
}
