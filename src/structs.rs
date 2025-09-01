pub fn main(){
    let v = MyVector{
        x: 5.0,
        y: 5.0
    };

    let mut v2 = MyVector::new(1.0);

    show_vector(&v);
    show_vector(&v2);
    v2.increase();
    show_vector(&v2);
}

fn show_vector(v: &MyVector){
    println!("({0}, {1})", v.x, v.y);
}

struct MyVector{
    x: f64,
    y: f64

}

impl MyVector{

    fn new(val: f64) -> MyVector{
        return MyVector{
            x: val,
            y: val
        }
    }

    fn increase(&mut self){
        self.x += 1.0;
        self.y += 1.0;
    }
}