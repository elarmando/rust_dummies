pub fn solve(){
    test(&String::from("test"));
    test(&String::from("test 21"));
    test(&String::from(""));
    test(&String::from(" "));
    test(&String::from("test moon ")); //should be 4
}

pub fn test(s: &String){
    let res = calc_len_last_word(&s);

    println!("{s} = {res}");
}

pub fn calc_len_last_word(s: &String) -> i32{
    if s.len() <= 0 {
        return 0;
    }

    let mut i: i32 = (s.len() - 1) as i32;
    let mut cnt = 0;
    let mut c = s.chars().nth(i as usize);

    while i >= 0 && c == Some(' '){
        i = i - 1;
         c = s.chars().nth(i as usize);
    }

    if i >= 0{
        c = s.chars().nth(i as usize);

        while i >= 0 && c != Some(' ') {
            cnt = cnt + 1;
            i = i - 1;

            c = s.chars().nth(i as usize);
        }
    } 

    return cnt;
}