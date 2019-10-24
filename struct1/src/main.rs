fn main() {
    println!("Hello, world!");

    struct User{
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

     let user1 = User{
         email: String::from("hi.aew@gmail.com"),
         username: String::from("aa"),
         sign_in_count: 1,
         active: false,
     };

    let mut user2 = User{
        email: String::from("hi.aaa@gmail.com"),
        sign_in_count: 2,
        active: true,
        username: String::from("ababa"),
    };

    user2.email = String::from("example@gmail.com");

    //结构体更新语法
    let user3 = User{
        email: user1.email,
        username: user2.username,
        active: user2.active,
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User{
        email: user2.email,
        ..user3
    };

//    元组结构体的定义和用法
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

struct User{
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 12,
    }
}