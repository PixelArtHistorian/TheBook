use rand::Rng;

fn main() {
    let x = rand::thread_rng().gen_range(-10..=10);

    if is_even(x) {
        println!("{x} is even");
    } else {
        println!("{x} is not even");
    }

    let y = abs(x);
    println!("The abs value of x is {y}");
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn abs(x: i32) -> i32 {
    if x >= 0 { x } else { -x }
}
