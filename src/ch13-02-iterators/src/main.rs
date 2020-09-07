fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // Use collect() to consumes the iterator and collects the resulting values into a collection data type
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
