fn main() {
    println!("Hello World!");
//
//    let mut s = String::from("hello world");
//    let word = first_world(&s);
//
//    s.clear();
//    println!("the first word is:{}", word);
//    let s = String::from("hello world");

////    包左不包右
//    let hello = &s[0..5];
//    let world = &s[6..11];
//
////    =意味着包含最后的数字
//    let hello1 = &s[0..=4];
//    let world1 = &s[6..=10];

    let my_string = String::from("hello world");

    let _word = first_world(&my_string[..]);

//    字符串字面值的形式
    let my_string_literal = "hello world";

//    传入字符串字面值的slice
    let _word1 = first_world(&my_string_literal[..]);
//    因为字符串字面值就是字符串slice，所以可以这样写，即不使用slice语法
    let _word2 = first_world(my_string_literal);
}

//fn first_word(s: &String) -> usize{
//
////    将String转换为字节数组
//    let bytes = s.as_bytes();
////使用iter方法在字节数组上创建一个迭代器, iter返回集合中的每一个元素，而enumerate包装了iter的结果，将这些元素作为元组的一部分来返回。
////enumerate返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用。
//    for (i, &item) in bytes.iter().enumerate(){
////        如果找到一个空格，返回它的位置，否则返回s.len()
//        if item == b' ' {
//            return 1;
//        }
//    }
//
//    s.len()
//}

// 这里传参改为传入&str,如果有一个字符串的slice可以直接传递它，如果有一个String，则可以传递整个String的slice。
//定义一个获取字符串Slice而不是String引用的函数使得我们的API更加通用并且不会丢失任何功能
//fn first_world(s: &String) -> &str{
fn first_world(s: &str) -> &str{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

//    返回s的切片
    &s[..]
}