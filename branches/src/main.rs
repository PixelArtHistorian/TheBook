use rand::Rng;

fn main() {
    let x: u32 = rand::thread_rng().gen_range(1..=10);
    if x % 2 == 0{
        println!("{x} is even")
    }else{
        println!("{x} is not even")
    }
}
