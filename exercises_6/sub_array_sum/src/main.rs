fn get_index_of_elements_sum(arr: &[u32], target: u32) -> Vec<i32> {
    let mut start = 0;
    let mut end = 0;
    let mut sum = 0;

    while end <= arr.len() {
        if sum == target {
            return vec![(start + 1) as i32, end as i32];
        } else if sum <= target {
            if end < arr.len() {
                sum += arr[end];
            }
            end += 1;
        } else {
            sum -= arr[start];
            start += 1;
        }
    }

    vec![-1]
}

fn main() {
    let mut input: String = String::new();
    println!("Enter elements");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Error while getting elements values");

    let arr: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().expect("Not a number"))
        .collect();

    let mut num: String = String::new();
    println!("Enter the target value");
    std::io::stdin()
        .read_line(&mut num)
        .expect("Error while reading the user input");

    let target: u32 = num.trim().parse::<u32>().expect("Not a number");

    let index: Vec<i32> = get_index_of_elements_sum(&arr, target);
    println!("{:?}", index);
}
