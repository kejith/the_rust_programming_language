fn main() {
    scope_of_literals();
    scope_of_complex_types();
    string_data_type();
    ownership_move();
    deep_copy();

    let s = String::from("hello");
    takes_ownership(s);
    // invalid because ownership was passed to the function
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    // valid because of simple type and a copy was passed to the function
    println!("{}", x); 


    let s1 = gives_ownership();
    let s2 = s1; // move ownership
    let s3 = takes_and_gives_ownership_back(s2);
    println!("{}", s3);
}

fn takes_and_gives_ownership_back(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn scope_of_literals() {
    let s = "outer";
    {
        println!("{}", s);
        let s = "inner";
        println!("{}", s);
    }
    
    println!("{}", s);
}


fn scope_of_complex_types() {
    let s = String::from("outer");
    {
        println!("{}", s);
        let s = String::from("inner");
        println!("{}", s);
    }
    
    println!("{}", s);
}

fn string_data_type() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s)
}

fn ownership_move() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 will be invalidated here

    // this would produce a compiler error "borrow of moved value: s1"
    // println!("{}, world!", s1);
    
    println!("{}, world!", s2);
}

fn deep_copy() {
    let s1 = String::from("hello");
    // copy stack and heap data
    // more expensive
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

