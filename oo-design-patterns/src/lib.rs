pub struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post{
        Post{
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str{
        self.state.as_ref().unwrap().content(&self)
    }

//    增加审核博文的功能
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review())
        }
    }
//    与request_review方法类似，会将state设置为审核通过时应处于的状态
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve());
        }
    }
}

//定义了不同状态的博文所共享的行为，同时Draft、PendingReview、Published状态都会实现State状态
trait State{
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str{
        ""
    }
}

//草稿
struct Draft{}

impl State for Draft{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
//        返回一个新的PendingReview实例，用来代表博文处于等待审核状态
        Box::new(PendingReview{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

//审核
struct PendingReview{}

impl State for PendingReview{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
//        不进行任何状态转换，只返回自身，因为请求审核已经处于PendingReview状态的博文应该保持PendingReview状态
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

//发布
struct Published{}

impl State for Published{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str{
        &post.content
    }
}