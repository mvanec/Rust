mod posts;

use blog::Post;
use macros::*;

fn main() {
    println!("\n=========Running {}", function!());

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post2 = posts::Post::new();

    post2.add_text("I ate a salad for lunch today");

    let post2 = post2.request_review();

    let post2 = post2.approve();

    assert_eq!("I ate a salad for lunch today", post2.content());

    let s = vec!["a".to_string(), "b".to_string(), "c".to_string()];

    let b = concat_all(s.into_iter(), "d");
    for l in b {
        println!("L is {l}");
    }

    let mut s: Vec<Box<dyn Display>> = Vec::new();
    add_displayable(&mut s, "valley".to_string());
    for d in s {
        println!("d is {d}");
    }

    let mut v: Vec<Box<dyn Display>> = Vec::new();
    {
        let s = String::from("Hello world");
        add_displayable(&mut v, s);
    }
    println!("{}", v[0]);

    let mut v: Vec<Box<dyn Display>> = Vec::new();
    add_displayable(&mut v, 5);


}

fn concat_all<'a>(
    iter: impl Iterator<Item = String> + 'a,
    s: &'a str,
) -> impl Iterator<Item = String> + 'a {
    iter.map(move |s2| s2 + s)
}

use std::fmt::Display;
fn add_displayable<'a, T: Display + 'a>(v: &mut Vec<Box<dyn Display + 'a>>, t: T) {
    v.push(Box::new(t));
}
