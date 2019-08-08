mod guts {
  use romp::declare;

  declare! {
    class Person {
      name: String,
      age: i32,
    }
    class Employee extends Person {
      role: String,
    }
  }
}

use guts::*;

fn main() {
  test_person();
  test_employee();
}

fn test_person() {
  let mut p = Person::new("Bob".into(), 10);
  println!("Person {{ name: \"{}\", age: {} }}", p.name(), p.age());

  //let mut p = Person { name: "Bob".into(), age: 10 };
  //println!("Foo {{ name: \"{}\", count: {} }}", p.name, p.age);

  p.set_name("Jim".into());
  p.set_age(20);
  println!("{:?}", p);
}

fn test_employee() {
  let mut e = Employee::new("Batman".into(), 32, "Hero".into());
  println!(
    "Employee {{ name: \"{}\", age: {}, role: \"{}\" }}",
    e.name(),
    e.age(),
    e.role()
  );

  e.set_name("Penguin".into());
  e.set_age(42);
  e.set_role("Villian".into());
  println!("{:?}", e);
}
