fn main() {
    let a:bool=true;
    let b:bool=false;
    match a {
        false =>println!("its false"),
        true => println!("its true"),
    }   
    match b {
        true=>println!("its true"),
        false => println!("its false"),
    }
}

// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display