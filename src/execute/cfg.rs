#[test]
fn test_execute() {
    use crate::*;
    fn sum(data: &[i32]) -> i32 {
        data.iter().sum()
    }
    fn add_offset(data: &[i32], offset: i32) -> i32 {
        data.iter().map(|x| x + offset).sum()
    }
    let nums: Vec<i32> = vec![1, 2, 3];
    let total: i32 = execute!(sum, &nums);
    assert_eq!(total, 6);
    let total_with_offset: i32 = execute!(add_offset, &nums, 10);
    assert_eq!(total_with_offset, 36);
}
