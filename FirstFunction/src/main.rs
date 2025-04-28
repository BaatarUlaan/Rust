fn main() {
    println!("Hello, world!");
    your_first_function();
}

fn your_first_function() {
    println!("your first function");
}

fn add_two_nubmer(a:i32,b:i32)->i32{
    let result=a+b;
    println!("a+b={}",result);
    
    //return result;
    5
}

fn exmaple_for_ic(a:i32){
    if a>0 {
        println!("Positive");
    }
    else if a<0{
        println!("Negative");
    }
    else{
        println!("Zero");
    }
    
    //Rust取缔了三元表达式（ternary expression）
    let condition=true;
    let number=if condition{a}else { -a };
    
}
