fn main() {
    another_function(3);
    let w = 5;
    print_labeled_measurement(w, 'w');
    let h = calculate_height(w);
    print_labeled_measurement(h, 'h');
}

fn another_function(x: i32){
    println!("This function prints this argument: {x}.");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measure is : {value} {unit_label}");
}
fn calculate_height(w: i32) -> i32{
    w * 2
}
