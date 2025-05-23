fn main(){
    println!("{}", reverse(-123));
}


fn reverse(x: i32) -> i32{
    let mut val = x;
    let mut result = 0;
    while val != 0{
        let temp = val % 10;
        val  /= 10;
        if result > i32::MAX / 10 || (result == i32::MAX / 10 && temp > 7) {
            return 0;
        }
        if result < i32::MIN / 10 || (result == i32::MIN / 10 && temp < -8) {
            return 0;
        }
        result = result*10 + temp;
    }

    return result;
}
