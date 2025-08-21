fn longer<'a, 'b, 'o>(left: &'a str, right: &'b str) -> &'o str
where
    'a: 'o,
    'b: 'o,
{
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
    let result = longer(&left, &right);
    assert_eq!(right, result);
    assert_eq!(5, result.len());
}
