//// //use std::fs::File;
//// //
//// ////use std::fs::File;
//// ////
//// //////////fn main() {
//// //////////    println!("Hello, world!");
//// //////////}
//// ////////fn main() {
//// ////////    panic!("crash and burn");
//// ////////}
//// //////
//// //////enum Result<T, E>{
//// //////    Ok(T),
//// //////    Err(E),
//// //////}
//// //////
//// //////fn main() {
//// //////    let v = vec![1, 2, 3];
//// //////
//// //////    v[2];
//// //////}
//// ////
//// ////fn main() {
//// ////    let f : u32 = File::open("hello.txt");
//// ////}
//// ////
//// //fn main() {
//// //    let f = File::open("hello.txt");
//// //
//// //    let f = match f {
//// //        Ok(file) => file,
//// //        Err(error) => {
//// //            panic!("There was a problem opening the file: {:?}", error)
//// //        },
//// //    };
//// //}
//
//// use std::fs::File;
//// use std::io::ErrorKind;
//
//// //使用不同的方式处理不同的错误
//// fn main() {
////     let f = File::open("hello.txt");
//
////     let f = match f {
////         Ok(file) => file,
////         Err(error) => match error.kind() {
////             ErrorKind::NotFound => match File::create("hello.txt") {
////                 Ok(fc) => fc,
////                 Err(e) => panic!("Tried to create file but there was a problem {:?}", e),
////             },
////             other_error => panic!("There was a problem opening the file: {:?}", error),
////         },
////     };
//
//// }
//use std::fs::File;
//use std::io::ErrorKind;
//
//// 这里有好多match，match非常强大，但是也非常的基础，一个更老练的Rustacean可能会这么写
//fn main() {
//    let f = File::open("hello.txt").unwrap_or_else(|error| {
//        if error.kind() == ErrorKind::NotFound{
//            File::create("hello.txt").unwrap_or_else(|error| {
//                panic!("Problem creating the file: {:?}", error);
//            })
//        }else {
//            panic!("Problem opening the file: {:?}", error);
//        }
//    });
//}

pub struct Guess{
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Guess{
        if value < 1 || value > 100{
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

//        返回Guess
        Guess{
            value
        }
    }

//    返回Guess的value值
    pub fn value(&self) -> i32{
        self.value
    }
}

fn main() {

}