mod posts;

use std::any::Any;

#[allow(dead_code)]
#[allow(unused)]
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn as_any(&self) -> &dyn Any;
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros::*;

    #[test]
    fn test_add_text() {
        println!("\n=========Running {}", function!());
        let message = "Test post";
        let mut post = Post::new();
        post.add_text(&message);
        assert_eq!(&post.content, &message);
    }

    #[test]
    fn test_state_request_review() {
        println!("\n=========Running {}", function!());
        let message = "Test post";
        let mut post = Post::new();
        post.add_text(&message);
        post.request_review();

        #[allow(unused)]
        let actual: &PendingReview = match post.state.as_ref().unwrap().as_any().downcast_ref::<PendingReview>() {
            Some(b) => b,
            None => panic!("&actual isn't a PendingReview!"),
        };

        assert_eq!(post.content(), "");
    }

    #[test]
    fn test_state_approve_without_revieww() {
        println!("\n=========Running {}", function!());
        let message = "Test post";
        let mut post = Post::new();
        post.add_text(&message);
        post.approve();

        #[allow(unused)]
        let actual: &Draft = match post.state.as_ref().unwrap().as_any().downcast_ref::<Draft>() {
            Some(b) => b,
            None => panic!("&actual isn't a Draft!"),
        };

        #[allow(unused)]
        let actual: &Published = match post.state.as_ref().unwrap().as_any().downcast_ref::<Published>() {
            Some(b) => panic!("&actual shouldn't be a Published!"),
            None => &Published{},
        };

        assert_eq!(post.content(), "");
    }

    #[test]
    fn test_state_approve_with_revieww() {
        println!("\n=========Running {}", function!());
        let message = "Test post";
        let mut post = Post::new();
        post.add_text(&message);
        post.request_review();
        post.approve();

        #[allow(unused)]
        let actual: &Published = match &post.state.as_ref().unwrap().as_any().downcast_ref::<Published>() {
            Some(b) => b,
            None => panic!("&actual shouldn't be a Published!"),
        };
        assert_eq!(post.content(), message);
    }
}
