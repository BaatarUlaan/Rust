
fn main() { 
    let s=String::from("Dajiadouzai nali liaotian a?");
    let result=change(s);
    println!("{}",result);
    
    
    
    let a=String::from("have a try");
    let mut b=String::from("this is b");
    const C:bool=false;
    if C{
        b=a;
    }
    println!("{}",a);
}

fn change(mut s:String)->String{
    s.push_str("haha");
    s
}
