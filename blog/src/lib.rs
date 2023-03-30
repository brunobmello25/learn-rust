pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
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

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn add_text(self: Box<Self>, post: &mut Post, content: &str) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approved_calls: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(self: Box<Self>, post: &mut Post, content: &str) -> Box<dyn State> {
        post.content.push_str(content);
        Box::new(Draft {})
    }
}

struct PendingReview {
    approved_calls: u32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        if self.approved_calls >= 1 {
            return Box::new(Published {});
        }

        self.approved_calls += 1;

        return self;
    }

    fn reject(mut self: Box<Self>) -> Box<dyn State> {
        self.approved_calls = 0;

        Box::new(Draft {})
    }

    fn add_text(self: Box<Self>, _post: &mut Post, _content: &str) -> Box<dyn State> {
        self
    }
}

struct Published {}

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(self: Box<Self>, _post: &mut Post, _content: &str) -> Box<dyn State> {
        self
    }
}
