
pub fn main(){
    let mut v = vec![1, 2, 3];
    let s = sum_vec(&v); 

    print_vec(&v);
    println!("sum={s}");

    sum_num_vec(5, & mut v);
    println!("Sum 5");
    print_vec(&v);
}

pub fn print_vec(v: &Vec<i32>){

    let mut i = 0;
    for vi in v{
        println!("vec[{i}]={vi}");
        i = i + 1;
    }
}

pub fn sum_vec(v: &Vec<i32>) -> i32{
    let mut sum = 0;

    for vi in v{
        sum = vi + sum;
    }

    return sum;
}

pub fn sum_num_vec(num: i32, v : & mut Vec<i32>){

    for i in  v{
        *i = *i + num;
    }

}