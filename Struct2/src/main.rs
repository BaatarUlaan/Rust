#[derive(Debug)]
#[derive(Clone)] //Copy 依赖Clone
struct Ren{
    chang:u64,
    gao:u64,
    zhong:u64,
    mingzi:String
}

impl Ren{
    fn zengchangshu(&mut self){
        self.chang+=10;
    }
    fn zhuoai(mut self)->Self{
        self.zhong-=20;
        self
    }
}
fn main() {
    let mut jia=Ren{
        chang:64,
        gao:64,
        zhong:64,
        mingzi:String::from("GaGa")
    };
    Ren::zengchangshu(&mut jia);
    println!("{:#?}",jia);
    jia.zengchangshu();
    
    println!("{:#?}",jia);
    
}
