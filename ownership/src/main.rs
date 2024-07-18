fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    //println!("{s}"); --this line will generate an error since ownership of string s is passed to the
    // function 'takes_ownership'

    let x = 9;

    makes_copy(x); //integer with a fixed size at compile time so nothing special happens
}

fn takes_ownership(some_string:String) {
    println!("{some_string}")
    //once this gets executed, 'drop' is called and the memory at the backend allocated to s is freed
}

fn makes_copy(some_integer:i32) {
    println!("{some_integer}")
}