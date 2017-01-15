extern crate task2;

use task2::Vector2;

#[test]
fn vector_test() {
    let mut point : Vector2<i32> = Vector2::origin();
    assert_eq!(Vector2::<i32>{x: 0, y: 0}, point);

    point = Vector2::unit_x();
    assert_eq!(Vector2::<i32>{x: 1, y: 0}, point);

    point = Vector2::unit_y();
    assert_eq!(Vector2::<i32>{x: 0, y: 1}, point);

    point = point * 3;
    assert_eq!(Vector2::<i32>{x: 0, y: 3}, point);

    point = point + point;
    assert_eq!(Vector2::<i32>{x: 0, y: 6}, point);
}

