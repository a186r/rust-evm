struct CustomSmartPointer{
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer{
        data: String::from("my stuff")
    };

    let d = CustomSmartPointer{
        data: String::from("other stuff")
    };
//    使用drop()方法提前清理
    drop(c);
    println!("CustomSmartPointers created.");

}