mod guts {
  use romp::declare;

  declare! {
    class Person {
      name: String,
      age: i32,
    }
  }
}

use guts::*;

fn main() {
  let mut p = Person::new("Bob".into(), 10);
  println!("Person {{ name: \"{}\", age: {} }}", p.name(), p.age());

  //let mut p = Person { name: "Bob".into(), age: 10 };
  //println!("Foo {{ name: \"{}\", count: {} }}", p.name, p.age);

  p.set_name("Jim".into());
  p.set_age(20);
  println!("{:?}", p);
}
