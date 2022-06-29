fn main() {
        let a:i32=5;
        let b:i32=4;
        let ab:i32=abc(a,b);
        if ab==11 { 
        println!("11");}
        else if ab>11{
            println!("11'den buyuk");
        }
        else {
            println!("11'den kucuk");
        }
}   fn abc(a:i32,b:i32)->i32   
    {a+b}
