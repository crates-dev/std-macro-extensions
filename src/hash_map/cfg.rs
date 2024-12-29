#[test]
fn test_hash_map() {
    use crate::*;
    let my_map: HashMap<&str, i32> = hash_map!("a" => 1, "b" => 2);
    assert_eq!(my_map["a"], 1);
    assert_eq!(my_map["b"], 2);
}
