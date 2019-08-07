mod foo {
  use romp::class;

  class!(Foo {
    name: String,
    count: i32,
  });
}

fn main() {
  let mut foo = foo::Foo::new("asdf".into(), 10);
  println!("Foo {{ name: \"{}\", count: {} }}", foo.name(), foo.count());

  //let mut foo = foo::Foo { name: "asdf".into(), count: 10 };
  //println!("Foo {{ name: \"{}\", count: {} }}", foo.name, foo.count);

  foo.set_name("qwer".into());
  foo.set_count(20);
  println!("{:?}", foo);
}
