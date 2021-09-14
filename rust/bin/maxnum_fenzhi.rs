struct Status {
    //左边最大
    l_sum: i32,
    //右边最大
    r_sum: i32,
    //中间最大
    m_sum: i32,
    //总和
    i_sum: i32,
}
impl Status {
    fn new() -> Status {
        Status {
            l_sum: 1,
            r_sum: 1,
            m_sum: 1,
            i_sum: 1,
        }
    }
    fn push_new(&self, l: Status, r: Status) -> Status {
        fn max(l: i32, r: i32) -> i32 {
            if l > r {
                l
            } else {
                r
            }
        }
        //计算总和
        let i_summ: i32 = l.i_sum + r.i_sum;
        //左边最大与左总和和右左最大相比
        let l_summ: i32 = max(l.l_sum, l.i_sum + r.l_sum);
        //右最大加上右总和左最大相比
        let r_summ: i32 = max(r.r_sum, r.i_sum + l.r_sum);
        //左右中的最大值和右左中间最大值的和对比
        let m_summ: i32 = max(max(l.m_sum, r.m_sum), l.r_sum + r.l_sum);
        Status {
            l_sum: l_summ,
            r_sum: r_summ,
            m_sum: m_summ,
            i_sum: i_summ,
        }
    }
    fn get(&self, a: Vec<i32>, l: i32, r: i32) -> Status {
        //if left == right
        //then return itself
        if l == r {
            return Status {
                l_sum: a[l as usize],
                r_sum: a[l as usize],
                m_sum: a[l as usize],
                i_sum: a[l as usize],
            };
        }
        // m = (l + r)/2
        let m: i32 = (l + r) >> 1;
        let l_sub: Status = self.get(a.clone(), l, m);
        let r_sub: Status = self.get(a, m + 1, r);
        //retun push_new
        self.push_new(l_sub, r_sub)
    }
    fn max_sub_array(&self, nums: Vec<i32>) -> i32 {
        self.get(nums.clone(), 0, (nums.len() - 1) as i32).m_sum
    }
}

fn main() {
    let money: Vec<i32> = vec![
        -5, 8, -5, 1, 1, -3, 5, 5, -3, -3, 6, 4, -7, -4, -8, 0, -1, -6,
    ];
    let output = Status::new();
    println!("{}", output.max_sub_array(money));
}
