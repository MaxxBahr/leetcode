fn simplify_path(path: String) -> String{
    //remove multiple slashes
    //split string at slash
    let mut split_string= path.split('/');
    let mut result: Vec<&str> = Vec::new();
    //on ".." remove previous folder
    for value in split_string{
        if value == ".."{
            result.pop();
            continue;
        }
        if value == "." || value == ""{
        //"." can be removed completely
        continue;
        }
        result.push(value);
    }
    result.insert(0, "");
    if result.len() == 1{
        return "/".to_string();
    }
    result.join("/")
}

#[test]
fn test_simplify_path(){
    assert_eq!(simplify_path("/home/".to_string()), "/home".to_string());
    assert_eq!(simplify_path("/home//foo/".to_string()), "/home/foo".to_string());
    assert_eq!(simplify_path("/home/user/Documents/../Pictures".to_string()), "/home/user/Pictures".to_string());
    assert_eq!(simplify_path("/../".to_string()), "/".to_string());
}