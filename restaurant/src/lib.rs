//以mod关键字为起始，然后指定模块的名字，并且用花括号包围模块的主体，模块内还可以定义其他模块
mod front_of_house{
//    模块中还可以保存一些定义的其他项，比如结构体、枚举、常量、特性或者函数
    pub mod hosting{
        pub fn add_to_waitlist(){}

        fn seat_at_table(){}
    }

    mod serving{
        fn take_order(){}

        fn server_order(){}

        fn take_payment(){}
    }
}

mod back_of_house{

    pub enum Appetizer{
        Soup,
        Salad,
    }

//    定义一个公有结构体
    pub struct Breakfast {
        pub toast: String,
        seasonan_fruit: String,
    }

//    因为Breakfast具有私有字段，所以这个结构体需要提供一个公共的关联函数来构造示例Breakfast
//    这里我们命名为summer
//    如果Breakfast没有这样的函数，我们将无法在eat_at_restaurant中创建Breakfast实例，
//    因为我们不能再ear_at_restaurant中设置私有字段seasonal_fruit的值

//    与之相反，如果我们将枚举设为公有，则它的所有成员都将变为公有，我们只需要在enum前加上pub
    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonan_fruit: String::from("peaches"),
            }
        }
    }

//    厨师修正了一个订单
    fn fix_incorrect_order(){
        cook_order();
//    使用以 super 开头的相对路径从父目录开始调用函数
        super::serve_order();
    }

    fn cook_order(){}
}

fn serve_order(){}

pub fn eat_at_restaurant(){
//    绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
//    相对路径
    front_of_house::hosting::add_to_waitlist();

//    后台吃早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
//    改变想法，想吃点别的
//    因为toast字段是公有的，所以可以使用点号来随意读写toast字段
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
//    下面这行编译不通过，因为这个字段是私有的
//    meal.seasonal_fruit = String::from("blueberries");
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}