fn plus_one(digits: Vec<i32>)-> Vec<i32>{
    let mut result = digits.clone();
    let n = result.len();

    for i in (0..n).rev(){
        if result[i] != 9{
            result[i] = result[i] +1;
            return result;
        }else{
            result[i] = 0;
        }
    }
    result.insert(0,1);
    result
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_nine() {
        assert_eq!(plus_one(vec![9]), vec![1,0]);
    }

    #[test]
    fn test_basic(){
        assert_eq!(plus_one(vec![1,2,3]), vec![1,2,4] );
    }

    #[test]
    fn test_long(){
        assert_eq!(plus_one(vec![1,2,9]), vec![1,3,0] );
    }

    #[test]
    fn test_hundret(){
        assert_eq!(plus_one(vec![9,9]), vec![1,0,0]);
    }
}
