use rand::Rng;

fn main() {
    let x = rand::thread_rng().gen_range(1..=10);
    
    if is_even(x){
        println!("{x} is even")
    }else{
        println!("{x} is not even")
    }
}

fn is_even(x: i32) -> bool{
    x % 2 == 0
}