pub fn greeting(name: &str) -> String{
//    format!("Hello {}!", name)
    String::from("Hello!")
}

pub struct Guess{
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess{
//        if value < 1 || value > 100{
//            panic!("Guess value must be between 1 and 100, got {}.", value);
//        }

        if value < 1{
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess{
            value
        }
    }
}

pub struct Rectangle{
    length: u32,
    width: u32,
}

impl Rectangle{
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2 + 1
}

#[cfg(test)]
mod tests {
//    use crate::Rectangle;
//    测试模块是一个内部模块，可以使用super将外部模块导入内部模块
    use super::*;

    #[test] // 表明是一个测试函数
    fn it_works() {
        assert_eq!(2 + 2, 4); // assert_eq!宏断言
    }

    #[test]
    fn exploration(){
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
//        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{
            length: 8,
            width: 7
        };

        let smaller = Rectangle{
            length: 5,
            width: 1
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle{length: 8, width: 3};
        let smaller = Rectangle{length: 7, width: 2};
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two(){
        assert_ne!(4, add_two(2));
    }

    #[test]
    #[should_panic]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(200);
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        }else{
            Err(String::from("two plus two does not equal four"))
        }
    }
}
