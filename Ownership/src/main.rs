
fn main() { 
    let s=String::from("Dajiadouzai nali liaotian a?");
    let result=change(s);
    println!("{}",result);
}

fn change(mut s:String)->String{
    s.push_str("haha");
    s
}
