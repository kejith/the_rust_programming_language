fn main() {
    print_labeled_measurement(5, 'h');
    codeblocks_are_expressions()
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

fn codeblocks_are_expressions() {
    let y = {
        let x = 3;
        // expressions do not include an ending semicolon
        // if you would use a semicolon here the expression
        // would turn into a statement and won't return its value
        x + 1 
    };

    println!("The value of y is: {y}");
}