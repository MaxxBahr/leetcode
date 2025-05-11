pub fn my_sqrt(x: i32) -> i32{
    let mut temp = 0;
    if x <= 1{
        return 0;
    }
    for i in (0..x){
        if i * i <= x{
            temp = i;
        }
    }
    temp
}

#[test]
fn simpletest(){
    assert_eq!(my_sqrt(4),2 );
}

#[test]
fn advancedtest(){
    assert_eq!(my_sqrt(8), 2);
}

#[test]
fn onetest(){
    assert_eq!(my_sqrt(1),0 );
}

#[test]
fn hardtest() {
    assert_eq!(my_sqrt(2147395599),46339 );
}
