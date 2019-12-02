#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn add_one_works(){
        assert_eq!(add_one(2), 3);
    }
}



pub fn add_one(x: i32) -> i32{
    x + 1
}