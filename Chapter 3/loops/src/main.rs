fn main() {
    break_with_return();
    labeled_loops();
    while_loops();
    array_iteration_for_loop();
    countdown_with_for_loop();
}

fn break_with_return() {
    println!("break_with_return() ---- start ------");
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result of break_with_return() is {result}");
    
    println!("break_with_return() ---- ends  ------");
}

fn labeled_loops() {
    println!("labeled_loops() ---- start ------");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
    
    println!("labeled_loops() ---- ends  ------");
}

fn while_loops() {
    println!("while_loops() ---- starts ------");
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");
    println!("while_loops() ---- ends  ------");
}

fn array_iteration_for_loop() {
    println!("array_iteration_for_loop() ---- starts ------");
    
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    println!("array_iteration_for_loop() ---- ends  ------");
}

fn countdown_with_for_loop() {
    println!("countdown_with_for_loop() ---- starts ------");
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
    println!("countdown_with_for_loop() ---- ends  ------");
}