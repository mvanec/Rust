const TWO: u32 = 1 + 1;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("{TWO}");

    // Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let mut a: u32 = 1;
    {
      let mut a = a;
      a += 2;
    }
    println!("{a}");
  }
