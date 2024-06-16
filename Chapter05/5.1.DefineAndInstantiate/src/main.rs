#[derive(Clone)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Point { x: i32, y: i32 }

fn print_point(p: &Point) {
    println!("{}, {}", p.x, p.y);
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1.clone()
    };
    println!("Value of user1.username: {}", user1.username);

    let mut p = Point { x: 0, y: 0 };
    let x = &mut p.x;
    print_point(&p);
    //*x += 1;

    let mut a = Point { x: 1, y: 2 };
    a.x += 1;
    let b = Point { y: 1, ..a };
    a.x += 1;
    println!("{}", b.x);

    p = Point { x: 1, y: 2 };
    let x = &mut p.x;
    let y = &mut p.y;
    *x += 1;
    *y += 1;
    println!("{} {}", p.x, p.y);
  }
