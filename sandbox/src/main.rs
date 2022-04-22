trait AsJson {
    fn as_json(&self) -> String;
}

struct Person {
    name: String,
    age: u8,
    favorite_fruit: String,
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool,
}

impl AsJson for Person {
    fn as_json(&self) -> String {
        format!(
            r#"{{"type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
            self.name, self.age, self.favorite_fruit
        )
    }
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
            self.name, self.color, self.likes_petting
        )
    }
}

// fn main() {
//     let mut counter = Counter::new(6);

//     assert_eq!(counter.next(), Some(1));
//     assert_eq!(counter.next(), Some(2));
//     assert_eq!(counter.next(), Some(3));
//     assert_eq!(counter.next(), Some(4));
//     assert_eq!(counter.next(), Some(5));
//     assert_eq!(counter.next(), Some(6));
//     assert_eq!(counter.next(), None);
//     assert_eq!(counter.next(), None);
//     assert_eq!(counter.next(), None);

//     counter = Counter::new(6);

//     for (i, c) in counter.enumerate() {
//         assert_eq!(c, i + 1);
//     }

//     let sum_until_10: usize = Counter::new(10).sum();
//     assert_eq!(sum_until_10, 55);

//     let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
//     assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);

//     let is_true = Counter::new(8).map(|n| 2usize.pow(n as u32)).any(|x| x > 1);
//     assert_eq!(is_true, true);
// }

struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
}

fn main() {
    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(
        Container::new(String::from("Bar")).value,
        String::from("Bar")
    );
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));

    let c: Container<char>;
    c = Container::new('1');
    c;
}
