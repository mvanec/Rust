struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    listing_15_14();
    listing_15_16();
    q1();

    // _s is dropped in the 3 below scenarios
    let mut _s = String::new();
    // (|_| ())(_s);
    // { _s; };
    drop(_s);
}

fn listing_15_14() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

}

// Drop an object before it goes out of scope
fn listing_15_16() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

struct Example(i32);
impl Drop for Example {
    fn drop(&mut self) {
        self.0 += 1;
        println!("drop {}", self.0);
    }
}
fn q1() {
    let e = Example(0);
    drop(e);
    //drop(e);
}
