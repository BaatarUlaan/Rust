struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_x(&mut self) -> &mut i32 { //相当于默认了声明周期相同：返回了对p的可变引用
        //fn get_x<'a>(&'a mut self) -> &'a mut i32
        //注意此处隐式地声明了生命周期
        &mut self.x
    }
}

fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = p.get_x();
    *x += 1;
    println!("{} {}", *x, p.y);
}