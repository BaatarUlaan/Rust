fn main() {
    let mut c=1;
    let a=loop{
        c += 2;
        
        if c>12 {break c;}//loop 循环返回c
    };
    println!("{}",a);
    
    //对loop使用标签吧
    'loop_label:loop{
        loop{
            println!("Reach!");
            break 'loop_label;
        }
        println!("Reach?");
    }
    
    
    let mut z=1;
    let w= while z==3{
        
        z+=1;
        break;
    };
    println!("{:?}",w);
}
