fn main() {
    let mut a=create_fstruct(String::from("first account"));
    let b =Fstruct{
        username:String::from("second account"),
        ..a //b 中只有username和a不同
    };

    a.liked_post.push(String::from("abc"));
    a.id=2;
    
}

fn create_fstruct(username:String)->Fstruct{
    let r=Fstruct{
        id:1,
        username,
        friend_list:vec![String::from("IShowSpeed"),String::from("xqc")],
        post:vec![String::from("Hey chat")],
        liked_post:vec![String::from("Hey chat")]
    };
    r
}

struct Fstruct{
    id:usize,
    username:String,
    friend_list:Vec<String>,
    post:Vec<String>,
    liked_post:Vec<String>,
}
