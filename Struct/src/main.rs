fn main() {
    let a=create_fstruct(String::from("first account"));
    let b =Fstruct{
        username:String::from("second account"),
        ..a
    };
    
    
    
}

fn create_fstruct(username:String)->Fstruct{
    let r=Fstruct{
        username,
        friend_list:vec![String::from("IShowSpeed"),String::from("xqc")],
        post:vec![String::from("Hey chat")],
        liked_post:vec![String::from("Hey chat")]
    };
    r
}

struct Fstruct{
    username:String,
    friend_list:Vec<String>,
    post:Vec<String>,
    liked_post:Vec<String>,
}
