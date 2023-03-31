pub mod annotate {
    const ROWS:i8 = 4;
    const COLS:i8 = 4;
    

    pub fn annotate(minefield: &[&str]) -> Vec<String> {
        // unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
        let mut res = Vec::<String>::new();
        let mut mine_pos = Vec::<i8>::new();

        for ind_char in minefield[0].char_indices() {

            if ind_char.1 == '*' {
                println!("{:?}", ind_char);
                mine_pos.push(ind_char.0.try_into().unwrap());
            }
        };

        for ind_char in minefield[0].char_indices() {
        
            if ind_char.1 == ' ' {
                let r = search_mine(ind_char.0.try_into().unwrap(), &mine_pos);
                if r != 0 {
                    res.push(r.to_string());
                } else {
                    res.push(' '.to_string());
                }
            } else {
                res.push('*'.to_string());
            }
        
        }
        return res;
    }

    fn search_mine(pos: i8, minefield: &Vec<i8>) -> i8 {

        
        let c = pos%COLS;

        println!("c: {}%{} = {}", pos, COLS, c);

        launch_search(c, pos, minefield)

        // let res:i8;
        // match c {
        //     0 => {
        //         res = search_up_right(pos, minefield) + 
        //         search_right(pos, minefield) + search_up(pos, minefield) + 
        //         search_down(pos, minefield) + search_down_right(pos, minefield)
        //     },
        //     1 => {
        //         res = search_up_right(pos, minefield) + 
        //         search_right(pos, minefield) + search_up(pos, minefield) + 
        //         search_down(pos, minefield) + search_down_right(pos, minefield) +
        //         search_left(pos, minefield) + search_down_left(pos, minefield) +
        //         search_up_left(pos, minefield)
        //     },
        //     3 => {
        //         res = search_up_right(pos, minefield) + 
        //         search_right(pos, minefield) + search_up(pos, minefield) + 
        //         search_down(pos, minefield) + search_down_right(pos, minefield) +
        //         search_left(pos, minefield) + search_down_left(pos, minefield) +
        //         search_up_left(pos, minefield)
        //     }
        //     2 => {
        //         res = search_up(pos, minefield) + 
        //         search_down(pos, minefield) + search_left(pos, minefield) + 
        //         search_down_left(pos, minefield) + search_up_left(pos, minefield)
        //     },
        //     _ => panic!("\nPosition unreachabe!!\n")
        // }
    }

    pub fn set_rows_and_cols(rows: u8, cols: u8) -> () {
        let r = rows;

    }

    fn launch_search(col: i8, pos: i8, minefield: &Vec<i8>) -> i8 {
        let mut res:i8 = 0;
        let tt = COLS-1;
        match col {
            0 => {
                res = search_up_right(pos, minefield) + 
                search_right(pos, minefield) + search_up(pos, minefield) + 
                search_down(pos, minefield) + search_down_right(pos, minefield);
            }, 

            tt => {
                res = search_up(pos, minefield) + 
                search_down(pos, minefield) + search_left(pos, minefield) + 
                search_down_left(pos, minefield) + search_up_left(pos, minefield);           
            } ,

            _ => {
                res = search_up_right(pos, minefield) + 
                search_right(pos, minefield) + search_up(pos, minefield) + 
                search_down(pos, minefield) + search_down_right(pos, minefield) +
                search_left(pos, minefield) + search_down_left(pos, minefield) +
                search_up_left(pos, minefield);
            }
        }

        return res;

    }

    fn search_up_right(pos: i8, stars: &Vec<i8>) -> i8 {
        if pos-COLS-1 >= 0 {
            if stars.contains(&(pos-COLS+1)) {
                return 1;
            }
        }
        return 0;
    }

    fn search_up_left(pos: i8, stars: &Vec<i8>) -> i8 {
        if pos-COLS >= 0 {
            if stars.contains(&(pos-COLS-1)) {
                return 1;
            }
        } 
        return 0;
    }

    fn search_down_right(pos: i8, stars: &Vec<i8>) -> i8 {
        if pos+COLS < ROWS*COLS{
            if stars.contains(&(pos+COLS+1)) {
                return 1;
            }
        }
        return 0;
    }

    fn search_down_left(pos: i8, stars: &Vec<i8>) -> i8 {
        if pos+COLS < ROWS*COLS{
            if stars.contains(&(pos+COLS-1)) {
                return 1;
            }
        }
        return 0;
    }

    fn search_right(pos: i8, stars: &Vec<i8>) -> i8 {
        if stars.contains(&(pos+1)) {
            return 1;
        } else {
            return 0;
        }
    }

    fn search_left(pos: i8, stars: &Vec<i8>) -> i8 {
        if stars.contains(&(pos-1)) {
            return 1;
        } else {
            return 0;
        }
    }

    fn search_down(pos: i8, stars: &Vec<i8>) -> i8 {
        if pos+COLS < ROWS*COLS{
            if stars.contains(&(pos+COLS)) {
                return 1;
            }
        }
        return 0;
    }

    fn search_up(pos: i8, stars: &Vec<i8>) -> i8 {
        if pos-COLS >= 0 {
            if stars.contains(&(pos-COLS)) {
                return 1;
            }
        } 
        return 0;
    }
}
