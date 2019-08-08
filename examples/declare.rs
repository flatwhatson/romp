mod guts {
  use romp::declare;

  declare! {
    class Person {
      name: String,
      age: i32,
    }
    class Character extends Person {
      role: String,
    }
    class Superhuman extends Character {
      power: String,
    }
  }
}

use guts::*;

fn main() {
  test_person();
  test_character();
  test_superhuman();
}

fn test_person() {
  let mut x = Person::new("Bob".into(), 10);
  println!("Person {{ name: \"{}\", age: {} }}", x.name(), x.age());

  //let mut x = Person { name: "Bob".into(), age: 10 };
  //println!("Foo {{ name: \"{}\", count: {} }}", x.name, x.age);

  x.set_name("Jim".into());
  x.set_age(20);
  println!("{:?}", x);
}

fn test_character() {
  let mut x = Character::new("Batman".into(), 32, "Hero".into());
  println!(
    "Character {{ name: \"{}\", age: {}, role: \"{}\" }}",
    x.name(),
    x.age(),
    x.role(),
  );

  x.set_name("Penguin".into());
  x.set_age(42);
  x.set_role("Villian".into());
  println!("{:?}", x);
}

fn test_superhuman() {
  let mut x = Superhuman::new("Hulk".into(), 32, "Hero".into(), "Strength".into());
  println!(
    "Superhuman {{ name: \"{}\", age: {}, role: \"{}\", power: \"{}\" }}",
    x.name(),
    x.age(),
    x.role(),
    x.power(),
  );

  x.set_name("Magneto".into());
  x.set_age(42);
  x.set_role("Villian".into());
  x.set_power("Magnetism".into());
  println!("{:?}", x);
}
