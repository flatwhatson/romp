use romp::class;

class!(Foo {
  name: String,
  count: i32,
});

fn main() {
  let mut foo = Foo { name: "asdf".to_string(), count: 10 };
  println!("Foo {{ name: \"{}\", count: {} }}", foo.name(), foo.count());

  foo.set_name("qwer".to_string());
  foo.set_count(20);
  println!("{:?}", foo);
}
