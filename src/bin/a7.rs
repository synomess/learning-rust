enum Sozluk {
    Dahi,
    Mal
}
fn main() {
    let go:Sozluk= Sozluk::Mal;
    match go {
        Sozluk::Dahi=>println!("meslek gruplarından çıkacağı gibi en üst sınır kabul edilir"),
        Sozluk::Mal=>println!("Saydemr")
    }
}

// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print