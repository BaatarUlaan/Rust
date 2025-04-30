fn main() {
    let x=String::from("haha");
    println!("Hello, world! {}",x);
    println!("In x:{}",x);

    let mut x = Box::new(5);

    let r1 = &mut x;
    println!("{:?}", r1);  // 使用了 r1

    let r2 = &x;
    println!("{:?}", r2);  // 同时使用了 r2
}
