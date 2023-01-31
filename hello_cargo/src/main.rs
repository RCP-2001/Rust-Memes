struct User{
    active: bool,
    username : String,
    email: String,
    sign_in_cout: u64,
}

impl User{
    
}

struct Color(i32,i32,i32);



fn main() {
    println!("Hello, world!");

    let s = "hello!";
    println!("{}", s);

    let mut s2 = String::from("hello");
  //  s. (", world!");pushStr
    
    println!("{}", s2);

    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_cout: 1,
    };

    let black = Color(0,0,0);


    let x : i32 = test();
    println!("{}", x);
}
//Test

fn test()->i32{
    let x : i32 = 1;
    return x;
}

fn Struct() {

}