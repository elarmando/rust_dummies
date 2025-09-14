pub fn main(){
    println!("game of life");
    let mut g = GameOfLife::new();

    let rows = g.get_rows();
    let cols = g.get_cols();

    let n = g.get(0,0);
    let val2 = g.get(1,1);

    println!("rows {rows} cols {cols} val {n} val2 {val2}");
}

struct GameOfLife{
    state: [[i32; 3];3]
}

impl GameOfLife{
    fn new()-> GameOfLife{
        return GameOfLife{
            state: [
                [0, 0, 0],
                [0, 1, 0],
                [0, 0, 0]
            ]
        };
    }

    fn get(&mut self, row: usize, col: usize)-> i32{
        return self.state[row][col];
    }

    fn get_rows(&mut self)-> usize{
        return self.state.len();
    }

    fn get_cols(&mut self) -> usize{
        if self.state.len() > 0{
            return self.state[0].len();
        }

        return 0;
    }
}