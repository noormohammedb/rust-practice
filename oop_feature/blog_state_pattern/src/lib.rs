mod oo_design;
mod state_as_type;

use state_as_type::Post;

pub fn run_ood() {
    let mut post = oo_design::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.requrest_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub fn run_state_as_type() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.requrest_review();

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
