use traits_objs::{Button, Draw, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// use std::fmt::Debug;
// fn q1() {
//     let n = 1;
//     let s = String::from("Hello");
//     let v: Vec<&dyn Debug> = vec![&n, &s];
//     let n_ref = v[0] as &i32;
//     println!("{}", n_ref + 1);
// }

#[allow(unused)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
