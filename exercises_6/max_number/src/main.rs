fn main() {
    let arr: [i32; 5] = [2, 1, 3, 2, 4];
    let mut f_max: i32 = arr[0];
    let mut s_max: i32 = arr[0];

    for i in &arr {
        if *i > f_max {
            s_max = f_max;
            f_max = *i;
        }
    }

    println!("second max number:{}", s_max);
}
