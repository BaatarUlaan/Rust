use std::io::stdin;
//我要使用std文件下的，io文件下的，stdin
fn main() {
    let mut input:String=String::new();
    //注意println的感叹号
    println!("Please input the message:");
    stdin().read_line(&mut input).unwrap();
    println!("The message is: {}",input.trim());
}
