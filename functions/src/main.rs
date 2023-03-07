fn main() {
    another_function(3);

    print_labeled_measurement(5, 'w');
}

fn another_function(x: i32){
    println!("This function prints this argument: {x}.");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measure is : {value} {unit_label}")
}
