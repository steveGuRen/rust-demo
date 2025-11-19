fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    // 冒泡排序
    for i in 0..len {
        // 每一轮把最大的放到最后
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut nums = vec![5, 3, 8, 4, 2, 1];

    println!("排序前: {:?}", nums);

    bubble_sort(&mut nums);

    println!("排序后: {:?}", nums);
}
