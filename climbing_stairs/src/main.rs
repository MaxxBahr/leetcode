pub fn climb_stairs(n: i32) -> i32 {
    let (mut a, mut b) = (1, 1);
    for _ in 0..(n-1) {
        let temp = a;
        a = b;
        b = temp + b;
    }
    b

}

#[test]
fn test_climb_stairs() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
    assert_eq!(climb_stairs(4), 5);
    assert_eq!(climb_stairs(5), 8);
    assert_eq!(climb_stairs(6), 13);
    assert_eq!(climb_stairs(7), 21);
    assert_eq!(climb_stairs(8), 34);
}
