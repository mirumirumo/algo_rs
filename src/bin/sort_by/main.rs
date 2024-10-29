fn main() {
    let mut v = vec![3, 4, 2, 1, 9, 5, 0];
    v.sort_by(|a, b| a.cmp(b));
    println!("{:?}", v);
    v.sort_by(|a, b| a.cmp(b).reverse());
    println!("{:?}", v);

    let mut v = vec![3.0, 4.0, 2.0, 1.0, 9.0, 5.0, 0.0];
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", v);

    let mut v = vec![(3, 4), (2, 1), (9, 5), (0, 7)];
    v.sort_by_key(|x| x.1);
    println!("{:?}", v);
    v.reverse();
    println!("{:?}", v);
}
