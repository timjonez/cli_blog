mod rs_lib;

fn main() {
    let mut post = rs_lib::Post::new();
    post.add_text("I think that rust is cool");
    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();
    assert_eq!("I think that rust is cool", post.content());
    println!("{}", post.content());
}
