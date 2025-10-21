pub fn main(){
    let myVec = vec![10.0, 12.0, 23.0, 23.0, 16.0, 23.0, 21.0, 16.0];
    let dev = standar_dev(&myVec);

    println!("4.8989794855664={dev}");
}

pub fn standar_dev(data: &Vec<f64>)-> f64 {

    if data.len() <= 0{
        return 0.0;
    }

    let mut sum: f64 = 0.0;
    let avg: f64 = avg(data);

    println!("avg {avg}");

    for n in data{
        sum = sum + (n - avg).powf(2.0); 
    } 

    println!("sum {sum}");

    sum = sum / (data.len() as f64);
    return sum.sqrt();
}


pub fn avg(data: &Vec<f64>)->f64{

    if data.len() <= 0{
        return 0.0;
    }

    let mut sum = 0.0;

    for n in data{
        sum = sum + n;
    }

    return sum / (data.len() as f64);

}
