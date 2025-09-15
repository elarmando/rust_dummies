pub fn main(){
    test_cell_dies();
}

pub fn test_cell_dies(){
    /*
    Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    */
    let mut game = GameOfLife::new();

    println!("current state");
    game.print();
    game.evolve();

    println!("after evolve"); 
    game.print();

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

    fn get_rows(&self)-> usize{
        return self.state.len();
    }

    fn get_cols(&self) -> usize{
        if self.state.len() > 0{
            return self.state[0].len();
        }

        return 0;
    }

    fn evolve(&mut self) {
        let cols = self.get_cols() as i32;
        let rows = self.get_rows() as i32;

        for c in 0..cols{
            for r in 0..rows{
                self.evolve_cel(r, c);
            }
        }
    }

    fn evolve_cel(&mut self, row: i32, col:i32){
        let live_neigbours_count = self.count_live_neighbours(row, col);
        let tr = row as usize;
        let tc = col as usize;

        if self.state[tr][tc] == 1 {
            if live_neigbours_count <= 0{
                
                self.state[tr][tc] = 0;
            }
        }
    }

    fn count_live_neighbours(&self, row: i32, col: i32)-> i32{
        let mut live_neigbours_count = 0;
        let neighboursx: [i32;3] = [-1, 0, 1];
        let neighboursy: [i32;3] = [-1, 0, 1];

        for dy in neighboursy{
            for dx in neighboursx{
                let r = row + dy;
                let c = col + dx;
                let tr = r as usize;
                let tc = c as usize;

                if self.is_neighbour(row, col, r, c) && self.state[tr][tc] == 1 {
                    live_neigbours_count += 1; 
                }
            }
        }

        return live_neigbours_count;
    }

    fn is_neighbour(&self, current_row: i32, curret_col:i32, row:i32, col:i32)-> bool{
        if current_row == row && curret_col == col{
            return false;
        }
        
        let cols = self.get_cols() as i32;
        let rows = self.get_rows() as i32;

        if col < 0  ||  col >= cols{
            return false;
        }

        if row < 0 || row >= rows{
            return false;
        }

        return true;
    }

    fn print(&self){
        let cols = self.get_cols();
        let rows = self.get_rows();

        for r in 0..rows{
            for c in 0..cols{
                 let value = self.state[r][c];
                print!("{value},");
            }

             println!("");
        }
    }
}