use iterators_and_speed::Rectangle;

#[test]
fn area_of_a_2_by_2_rectangle_is_4() {
    let subject = Rectangle::new(2, 2);
    assert_eq!(subject.area(), 4);
}
