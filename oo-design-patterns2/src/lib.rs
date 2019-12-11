pub struct Post{
    content: String,
}

pub struct DraftPost{
    content: String,
}

pub struct PendingReviewPost{
    content: String,
}

impl Post{
    pub fn new() -> DraftPost{
        DraftPost{
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str{
        &self.content
    }
}

impl DraftPost{
    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }

//    消费self的所有权，将DrafePost转换为PendingReviewPost，在调用这个方法之后不会遗留任何的DraftPost实例
    pub fn request_review(self) -> PendingReviewPost{
        PendingReviewPost{
            content: self.content,
        }
    }
}

impl PendingReviewPost {
//    消费self的所有权，将PendingReviewPost转换为Post
    pub fn approve(self) -> Post{
        Post{
            content: self.content,
        }
    }
}