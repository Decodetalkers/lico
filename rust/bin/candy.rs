//只要知道谷底，左峰，右峰，就可以通过公式计算出来糖果数量
struct Solution;
impl Solution {
    //状态机太可怕了，很难把所有状态都计算进去，所以失败了。
    fn candy(ratings: Vec<i32>) -> i32 {
        let len = ratings.len();
        if len == 1 {
            return 1;
        }
        let mut down: Vec<usize> = vec![];
        let mut left: Vec<usize> = vec![];
        let mut right: Vec<usize> = vec![];
        for (i, rat) in ratings.iter().enumerate() {
            if i == 0 {
                if rat <= &ratings[1] {
                    down.push(0);
                } else {
                    right.push(0);
                }
                continue;
            }
            if i == len - 1 {
                if rat <= &ratings[len - 2] {
                    down.push(i);
                } else {
                    left.push(i);
                }
                continue;
            }
            if rat <= &ratings[i - 1] && rat <= &ratings[i + 1] {
                down.push(i);
            } else if rat > &ratings[i - 1] && rat >= &ratings[i + 1] {
                left.push(i);
            } else if rat > &ratings[i + 1]
                && (rat >= &ratings[i - 1] || (i > 2 && ratings[i - 1] > ratings[i - 2]))
            {
                right.push(i);
            }
        }
        //初始糖果一颗
        let mut output = 0;
        //println!("{:?}\n{:?}\n{:?}",down,left,right);
        let mut down = down.into_iter();
        let mut left = left.into_iter();
        let mut right = right.into_iter();
        let mut down2 = down.next();
        let mut left2 = left.next();
        let mut right2 = right.next();
        loop {
            match (down2, left2, right2) {
                (None, None, None) => break output,
                // 一直是谷底了，那就一直加一就好了
                (Some(_), None, None) => {
                    output += 1;
                    down2 = down.next();
                }
                //断层是不可能脱离谷底存在的
                (None, _, _) => {
                    println!("sss");
                }
                // 右谷底有右断层，这时候就是只剩下一个下坡路了,左山峰右谷底
                (Some(a), None, Some(c)) => {
                    if a < c {
                        output += 1;
                        down2 = down.next();
                        continue;
                    }
                    //println!("sss,{}",a);
                    let local_len = a - c + 1;
                    output += ((local_len * (local_len + 1)) / 2) as i32;
                    down2 = down.next();
                    right2 = right.next();
                }
                // 左谷底右山峰
                (Some(a), Some(b), None) => {
                    let mut temp = down.clone();
                    let next_location = temp.next();
                    if next_location.is_some() && next_location.unwrap() < b {
                        output += 1;
                        down2 = down.next();
                        continue;
                    }
                    let local_len = b - a + 1;
                    output += ((local_len * (local_len + 1)) / 2) as i32;
                    down2 = down.next();
                    left2 = left.next();
                }
                // 存在两种布局,谷左右，右谷左，总之谷底必须在左峰前面
                (Some(a), Some(b), Some(c)) => {
                    // 谷左右，这时候应该计算谷底到左边的长度
                    // 左边同时计算万完成推进
                    //println!("a={},b={},c={}",a,b,c);
                    if b < c {
                        let mut temp = down.clone();
                        let next_location = temp.next();
                        if next_location.is_some() && next_location.unwrap() < b {
                            output += 1;
                            down2 = down.next();
                            continue;
                        }
                        let mut addon = 0;
                        // 如果左右两个峰是挨着的
                        if c - b == 1 {
                            let mut temp = down.clone();
                            let next_location = temp.next().unwrap();
                            // 这一步做分析，先判断跨度
                            // 如果跨度右边大，那么要把少的内容加到左边
                            // 相等就不需要判断
                            // 之后追加判断，如果是顶峰的话，需要额外加一，来形成最高点
                            if next_location > c && next_location - c > b - a {
                                addon = ((next_location - c) - (b - a)) as i32;
                                if ratings[b] > ratings[c] {
                                    addon += 1;
                                }
                            } else if next_location > c
                                && next_location - c == b - a
                                && ratings[b] > ratings[c]
                            {
                                addon += 1;
                            }
                        }
                        let local_len = b - a + 1;
                        output += ((local_len * (local_len + 1)) / 2) as i32 + addon;
                        down2 = down.next();
                        left2 = left.next();
                    // 右谷左，要计算右边到谷底内容
                    // 谷底和右边推进
                    } else {
                        if a < c {
                            output += 1;
                            down2 = down.next();
                            continue;
                        }
                        let local_len = a - c + 1;
                        output += ((local_len * (local_len + 1)) / 2) as i32 - 1;
                        //down2 = down.next();
                        right2 = right.next();
                    }
                }
            }
        }
    }
    fn candy2(ratings: Vec<i32>) -> i32 {
        //输出值
        let mut ans = 0;
        //全部赋值为1
        let mut nums = vec![1; ratings.len()];
        //从左到右，排序一遍，大的比前面大一
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                nums[i] = nums[i - 1] + 1;
            }
        }
        //由于最后一个不被计算，所以先加上
        ans += nums[nums.len() - 1];
        //从最后到左，排序，这样可以确定最大值
        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                nums[i] = nums[i].max(nums[i + 1] + 1);
            }
            ans += nums[i];
        }
        ans
    }
}
pub fn main() {
    println!("{}", Solution::candy(vec![1, 0, 2, 3, 4, 5]));
    println!("{}", Solution::candy2(vec![1, 0, 2, 3, 4, 5]));
    println!("{}", Solution::candy(vec![1, 3, 2, 2, 1]));
    println!("{}", Solution::candy(vec![1, 2, 2]));
    println!("{}", Solution::candy(vec![1, 6, 10, 8, 7, 3, 2]));
    println!("{}", Solution::candy(vec![1, 3, 2, 2, 1]));
    println!("{}", Solution::candy(vec![1, 2, 3, 5, 4, 3, 2, 1]));
    println!("{}", Solution::candy(vec![1, 2, 3, 1, 0]));
    println!(
        "{}",
        Solution::candy(vec![
            1, 2, 3, 5, 4, 3, 2, 1, 4, 3, 2, 1, 3, 2, 1, 1, 2, 3, 4
        ])
    );
    println!(
        "{}",
        Solution::candy(vec![
            1, 2, 3, 5, 4, 3, 2, 1, 4, 3, 2, 1, 3, 2, 1, 1, 2, 3, 4, 4, 3, 2, 1
        ])
    );
    println!("{}", Solution::candy(vec![3, 2, 1, 1, 4, 3, 3]));
}
