pub fn main(){
    iterate_matrix();
}

pub fn iterate_matrix(){
    let matrix: [[i32; 3]; 3] = [
        [1, 2, 1], 
        [2, 3, 2],
        [1, 2, 1]
        ];

    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows{

        for j in 0..cols{
            let num = matrix[i][j];
            print!("{num},");
        }

        println!("");
    }


}