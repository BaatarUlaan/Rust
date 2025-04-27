fn main() {
    let t=b'8';

    let a=1.222;
    
    let c=true;
    
    let u=0x123_222;
    
    //注意在Rust中使用0o，在C语言中使用0
    let i=0o123;
    let i=0234;
    
    
    let tup:(i32,u32,char)=(-1,2,'A');
    //一旦创建tuple,就不能变了
    //tup.0取出第一个元素
    
    let array:[i32;4] = [1,2,3,4];
    let array2=[3;5]; //3重复4次
    
}
