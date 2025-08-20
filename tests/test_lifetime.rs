fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

#[test]
fn test_lifetime() {
    let left = String::from("left");
    let right = String::from("right");
    assert_eq!(right, longer(&left, &right));
}
