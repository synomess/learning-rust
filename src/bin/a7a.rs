enum Color {
    mavi,
    kirmizi,
    mor,
            }
   fn print_color (my_color:Color) {
       match my_color {
           Color::kirmizi=>println!("kirmizi"),
           Color::mavi=>println!("mavi"),
           Color::mor=>println!("mor"),
       }
   }  fn main() {
    print_color(Color::mavi);
   }