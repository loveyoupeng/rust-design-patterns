/// Compares two string slices by length and returns a reference to the longer one.
/// If lengths are equal, it returns the `left` slice.
///
/// This function uses three lifetime parameters:
/// - `'a`: The lifetime of the `left` string slice.
/// - `'b`: The lifetime of the `right` string slice.
/// - `'o`: The output lifetime of the returned string slice.
///
/// The `where` clause ` 'a: 'o, 'b: 'o,` specifies that the output lifetime `'o`
/// must be contained within (or outlive) both `'a` and `'b`. This ensures that
/// the returned reference does not outlive either of the input references,
/// guaranteeing memory safety by ensuring the returned reference is valid
/// for the intersection of the input lifetimes.
///
/// # Arguments
///
/// * `left` - A string slice with lifetime `'a`.
/// * `right` - A string slice with lifetime `'b`.
///
/// # Returns
///
/// A string slice with lifetime `'o` which is either `left` or `right`,
/// depending on their lengths.
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
