#[test]
fn test_cell() {
    use crate::*;
    let cell_value: Cell<i32> = cell!(5);
    assert_eq!(cell_value.get(), 5);
    cell_value.set(10);
    assert_eq!(cell_value.get(), 10);
}
