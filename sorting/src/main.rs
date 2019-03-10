fn main() {
    println!("Rust data structure programming ~");
    println!("Bubble sorting ~\n");

    // value of elements is different
    let mut arr: [i32; 11] = [22, 12, 6, 88, 0, -5, 99, -1001, 12, 3, 66];

    // value of elements is the same
    //let mut arr: [i32; 11] = [6; 11];

    println!("Original order:");
    println!("{:?}", arr);

    let len = arr.len();

    // Bubble sorting
    /*for i in 0..len {
        println!("====== Round {} ======", i);
        println!("{:?}", arr);

        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                println!("swap <j>:{} <j+1>:{}", j, j + 1);

                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
            println!("{:?}", arr);
        }
    }*/

    // Selection sorting
    /*for i in 0..len {
        println!("====== Round {} ======", i);
        println!("{:?}", arr);
        let mut min = i;

        for j in i..len {
            if arr[min] > arr[j] {
                min = j;
            }
            println!("i:{} j:{} min:{} min_val:{}", i, j, min, arr[min]);
        }

        if arr[i] != arr[min] {
            println!("swap i:{} min:{}", i, min);
            let temp = arr[i];
            arr[i] = arr[min];
            arr[min] = temp;
        }
    }*/

    //Insertion sorting
    for i in 0..len {
        let mut index = i;
        println!("====== Round {} ======", i);
        for i in 0..len {
            if i == index {
                print!("{}, $$$  ", arr[i]);
            } else {
                print!("{}, ", arr[i]);
            }
        }
        print!("\n");

        for j in 0..i {
            if arr[i] < arr[j] {
                println!("swap i:{} j:{}", i, j);

                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
            println!("{:?}", arr);
        }
    }

    println!("Array is sorted:");
    println!("{:?}", arr);
}
