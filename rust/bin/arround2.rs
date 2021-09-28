// 从四条边进入，深度优先，把所有连一起的都标记上然后把没标记的O换掉就可以了
struct Solution;
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let lenx = board.len();
        let leny = board[0].len();
        let search_x = vec![false; leny];
        let mut search = vec![search_x; lenx];
        for i in 0..lenx {
            if board[i][0] == 'O' && !search[i][0] {
                search[i][0] = true;
                clear(i, 0, lenx, leny, board, &mut search);
            }
            if board[i][leny - 1] == 'O' && !search[i][leny - 1] {
                search[i][leny - 1] = true;
                clear(i, leny - 1, lenx, leny, board, &mut search);
            }
        }
        //let search_x = vec![false;leny];
        for j in 0..leny {
            if board[0][j] == 'O' && !search[0][j] {
                search[0][j] = true;
                clear(0, j, lenx, leny, board, &mut search);
            }
            if board[lenx - 1][j] == 'O' && !search[lenx - 1][j] {
                search[lenx - 1][j] = true;
                clear(lenx - 1, j, lenx, leny, board, &mut search);
            }
        }

        //println!("{:?}", search);
        for i in 1..lenx - 1 {
            for j in 1..leny - 1 {
                if board[i][j] == 'O' && !search[i][j] {
                    board[i][j] = 'X';
                }
            }
        }
    }
}

fn clear(
    x: usize,
    y: usize,
    lenx: usize,
    leny: usize,
    board: &[Vec<char>],
    search: &mut Vec<Vec<bool>>,
) {
    if x > 0 && board[x - 1][y] == 'O' && !search[x - 1][y] {
        search[x - 1][y] = true;
        clear(x - 1, y, lenx, leny, board, search);
    }
    if x + 1 < lenx && board[x + 1][y] == 'O' && !search[x + 1][y] {
        search[x + 1][y] = true;
        clear(x + 1, y, lenx, leny, board, search);
    }
    if y > 0 && board[x][y - 1] == 'O' && !search[x][y - 1] {
        search[x][y - 1] = true;
        clear(x, y - 1, lenx, leny, board, search);
    }

    if y + 1 < leny && board[x][y + 1] == 'O' && !search[x][y + 1] {
        search[x][y + 1] = true;
        clear(x, y + 1, lenx, leny, board, search);
    }
}
fn main() {
    let mut input = vec![
        vec!['O', 'X', 'O', 'O', 'X', 'X'],
        vec!['O', 'X', 'X', 'X', 'O', 'X'],
        vec!['X', 'O', 'O', 'X', 'O', 'O'],
        vec!['X', 'O', 'X', 'X', 'X', 'X'],
        vec!['O', 'O', 'X', 'O', 'X', 'X'],
        vec!['X', 'X', 'O', 'O', 'O', 'O'],
    ];
    Solution::solve(&mut input);
    println!("{:?}", input);
    let mut input = vec![
        vec!['X', 'O', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'O'],
        vec!['X', 'O', 'O', 'X', 'X', 'X', 'O', 'O', 'O', 'X'],
        vec!['O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'X', 'X'],
        vec!['O', 'O', 'O', 'O', 'O', 'O', 'X', 'O', 'O', 'X'],
        vec!['O', 'O', 'X', 'X', 'O', 'X', 'X', 'O', 'O', 'O'],
        vec!['X', 'O', 'O', 'X', 'X', 'X', 'O', 'X', 'X', 'O'],
        vec!['X', 'O', 'X', 'O', 'O', 'X', 'X', 'O', 'X', 'O'],
        vec!['X', 'X', 'O', 'X', 'X', 'O', 'X', 'O', 'O', 'X'],
        vec!['O', 'O', 'O', 'O', 'X', 'O', 'X', 'O', 'X', 'O'],
        vec!['X', 'X', 'O', 'X', 'X', 'X', 'X', 'O', 'O', 'O'],
    ];
    Solution::solve(&mut input);
    println!("{:?}", input);
}
