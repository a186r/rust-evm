//fn main() {
//    let number_list = vec![34, 50, 25, 100, 65];
//
//    let mut largest = number_list[0];
//
//    for number in number_list{
//        if number > largest{
//            largest = number;
//        }
//    }
//
//    println!("The largest number is {}", largest);
//}
//参数为i32类型列表的引用
fn largest(list: &[i32]) -> i32{
    let mut largest = list[0];

    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 45, 56, 200, 324];

    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let number_list = vec![123,234,455, 56,7];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}