pub fn main(){
    let my_arr: [i32; 5] = [1, 2, 3, 4, 5];
    let avg = find_avg(&my_arr);
    let min = find_min(&my_arr);
    let max = find_max(&my_arr);
    let sum = calc_sum(&my_arr);

    println!("avg = {avg}");
    println!("min = {min}");
    println!("max = {max}");
    println!("sum = {sum}");
}

pub fn find_avg(a: &[i32])-> f64{
    let mut res: i32 = 0;

    for num in a{
        res += num;
    }

    return (res as f64)  / (a.len() as f64);
}

pub fn find_min(a: &[i32]) -> i32{
    let mut min: i32 = 0;

    if a.len() > 0 {

        min = a[0];

        for num in a {
            if *num < min {
                min = *num;
            } 
        }

    }

    return min;
}

pub fn find_max(a: &[i32])->i32{
    let mut max: i32 = 0;

    if a.len() > 0 {
        for num in a {
            if *num > max {
                max = *num;
            }
        }
    }

    return max;
}

pub fn calc_sum(a: &[i32]) -> i32{
    let mut sum: i32 = 0;

    for num in a {
        sum = sum + *num;
    }

    return sum;
}

