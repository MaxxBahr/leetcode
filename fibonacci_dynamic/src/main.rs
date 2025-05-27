fn fibo(result: &mut Vec<i32>, n: i32)-> i32{
    if n <= 1{
        return n;
    }
    result.push(fibo(result,n-1)+fibo(result,n-2));
    return result[n as usize];
}

fn main(){
    let mut result: Vec<i32> = Vec::new();
    fibo(&mut result, 1);
    fibo(&mut result, 2);
}