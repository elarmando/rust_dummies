pub fn main(){
    println!("hola");
    test(1);
    test(2);
    test(3);
    test(4);
    test(5);
    test(6);
    test(7);
    test(8);
    test(2147395600); //46340
}

pub fn test(x: i32){
    let y = my_sqrt(x);
    println!("sqrt({x})={y}");
}

pub fn my_sqrt(x: i32) -> i32{
    let mut sol: i32 = 1;

    while sol <= x / sol{
        sol = sol + 1;
    }

    return sol - 1;
}