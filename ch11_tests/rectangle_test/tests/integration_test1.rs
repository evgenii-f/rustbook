use rectangle_test;

#[test]
fn test_rectangle() {
    let some = rectangle_test::Rectangle{width: 10., height: 20.};
    assert_eq!(10., some.width());
    assert_eq!(20., some.height());
}