pub fn main(){
    test_palindrome("hoh");
    test_palindrome("h");
    test_palindrome("ho");
}

pub fn test_palindrome(s: &str){
    let s1 = String::from(s);
    let res = is_palindrome(&s1);
    println!("{s1} is palindrome: {res}");
}

pub fn is_palindrome(s: &String) -> bool{
    let mut i: usize = 0;
    let mut j: usize = s.len() - 1;

    while i < j {
        let currenti = s.chars().nth(i);
        let currentj = s.chars().nth(j);

        if currenti != currentj {
            return false;
        }

        i = i + 1;
        j = j - 1;
    }


    return true;
}
