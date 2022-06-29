// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let  a:i32=24;
    let  b:i32=1;
    let mut ab:i32=abc(a, b);
    if ab==27 {
    println!("{}",ab=ab+1);}
    else if ab==25 {
        loop {
            ab = ab+5;
            println!("{}",ab+5);
            if ab+5==40 {break;}
    }
    }
    else{ println!("{}",ab=ab+2);}
    }fn abc(a:i32,b:i32)->i32 {a+b}
