
fn main() {
    let mut s = String::from("hello");
    
    
    //你不能在已经存在一个可变借用（&mut s）的同时，再创建对 s 的不可变借用（例如直接使用 s 本身）。
    let r1 = &mut s;
    //let r2 = &mut s;

    println!("{}, {}", r1, s);
}

fn try_the_case<'a>()-> &'a String{
    let s = String::from("hello");
    &s
}
