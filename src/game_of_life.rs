use rand::Rng;
use std::{thread, time::Duration};

pub fn main(){
    test_cell_dies();
}

pub fn test_cell_dies(){
    /*
    Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    */
    let mut game = GameOfLife::new(50);
    game.init();

    while(true){
        game.clear();
        game.print();
        game.evolve();

         thread::sleep(Duration::from_millis(500));
    }

}

struct GameOfLife{
    state: Vec<Vec<i32>>
}

impl GameOfLife{
    fn new(size: usize)-> GameOfLife{
        let mut state:  Vec<Vec<i32>> = Vec::new();

        for i in 0..size{
            state.push(Vec::new());

            for _j in 0..size{
                state[i].push(1);
            }
        }

        return GameOfLife{
            state :   state
        };
    }

    fn init(&mut self){
        let cols = self.get_cols() as i32;
        let rows = self.get_rows() as i32;

        for c in 0..cols{
            for r in 0..rows{
                 let num = rand::thread_rng().gen_range(0..100);
                 let row = r as usize;
                 let col = c as usize;

                 if num <= 49{
                    self.state[row][col] = 0;
                 }
                 else{
                    self.state[row][col] = 1;
                 }
            }
        }
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

        if self.state[tr][tc] == 1 { // live cell
            if live_neigbours_count < 2{
                //1 Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                
                self.state[tr][tc] = 0;
            }
            else if live_neigbours_count == 2 || live_neigbours_count == 3{
                //2 Any live cell with two or three live neighbours lives on to the next generation.
                self.state[tr][tc] = 1;
            }
            else if live_neigbours_count > 3 {
                //3 Any live cell with more than three live neighbours dies, as if by overpopulation.
                self.state[tr][tc] = 0;
            }
        }
        else{//dead cell
            if live_neigbours_count == 3{
                //4 Any live cell with more than three live neighbours dies, as if by overpopulation.
                self.state[tr][tc] = 1;
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

    fn clear(&self){
         print!("\x1B[2J\x1B[1;1H");
    }

    fn print(&self){
        let cols = self.get_cols();
        let rows = self.get_rows();

        for r in 0..rows{
            for c in 0..cols{
                 let value = self.state[r][c];

                 if value == 1{
                    print!("#");
                 }else{
                    print!(" ");
                 }
            }

             println!("");
        }
    }
}