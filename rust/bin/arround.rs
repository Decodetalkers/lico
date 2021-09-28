struct Solution;
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let lenx = board.len();
        let leny = board[0].len();
        let search_x = vec![false; leny];
        let mut search = vec![search_x; lenx];
        for j in 1..leny - 1 {
            for i in 1..lenx - 1 {
                if board[i][j] == 'O' && board[i - 1][j] != 'O' && board[i][j - 1] != 'O' {
                    clear(i, j, lenx, leny, &mut *board, &mut search);
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
    board: &mut Vec<Vec<char>>,
    search: &mut Vec<Vec<bool>>,
) -> bool {
    if x < lenx - 1
        && y < leny - 1
        && ((x != 1 && (!search[x - 1][y] || board[x - 1][y] == 'X'))
            || (x == 1 && board[0][y] != 'O'))
        && ((y != 1 && (!search[x][y - 1] || board[x][y - 1] == 'X'))
            || (y == 1 && board[x][0] != 'O'))
    {
        let need_clean = match (board[x + 1][y], board[x][y + 1]) {
            ('X', 'X') => true,
            ('X', 'O') => clear(x, y + 1, lenx, leny, &mut *board, &mut *search),
            ('O', 'X') => clear(x + 1, y, lenx, leny, &mut *board, &mut *search),
            (_, _) => {
                clear(x + 1, y, lenx, leny, &mut *board, &mut *search)
                    && clear(x, y + 1, lenx, leny, &mut *board, &mut *search)
            }
        };
        if need_clean {
            board[x][y] = 'X';
        }
        //println!("{:?}",search);
        search[x][y] = true;
        need_clean
    } else {
        search[x][y] = true;
        false
    }
}
pub fn main() {
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
}
