

struct Counter {

}

impl Iterator for Counter<T> {
    Item
    fn next() {

    }
}

fn main() {
    let mut counter = Counter::new(6);

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), Some(6));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);

    counter = Counter::new(6);

    for (i, c) in counter.enumerate() {
        assert_eq!(c, i + 1);
    }

    let sum_until_10: usize = Counter::new(10).sum();
    assert_eq!(sum_until_10, 55);

    let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);

    let is_true = Counter::new(8).map(|n| 2usize.pow(n as u32)).any(|x| x > 1);
    assert_eq!(is_true, true);
}
