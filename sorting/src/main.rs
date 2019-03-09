fn main() {
    println!("Rust data structure programming ~");
    println!("Bubble sorting ~\n");

    let mut arr: [i32; 7] = [22, 12, 6, 88, 45, 49, 99];

    println!("Original order:");
    println!("{:?}", arr);

    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
            println!("i:{} j:{}", i, j);
        }
    }

    println!("Array is sorted:");
    println!("{:?}", arr);
}
