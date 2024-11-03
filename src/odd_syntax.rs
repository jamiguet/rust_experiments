macro_rules! f {
  ($cond: expr) => {
    if let Some(x) = $cond {
      println!("i am some == {x}!");
    } else {
      println!("i am none");
    }
  }
}

pub fn main() {
    f!(Some(100));

    {
        f!(Some(100));
        return;

        const x: i32 = 5;
    }
}