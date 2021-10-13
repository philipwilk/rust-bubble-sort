fn main() {
    let mut array: [i32; 10] = [1, 3, 2, 5, 4, 52529, 4, 50, 30, 20];
    let mut loop_iterations = 0;
    let sorted = loop {
        loop_iterations += 1;
        println!("Iteration {}", loop_iterations);
        let mut noswitch = 0;
        for key in 0..array.len()-1 {
            // If the current key is the last key, add to noswitch 
            println!("Key is {}, value is {}", key, array[key]);
            if key+1 == array.len() {
                noswitch += 1;
            }
            // Otherwise, actually, check the value is bigger/smaller
            else {
                if array[key] > array[key+1] {
                    let prev = array[key];
                    array[key] = array[key+1];
                    array[key+1] = prev;
                }
                else {
                    noswitch += 1;
                }
            }
        };
        
        // If the number of switches that did not happen is equal to the array lenth(-1), then the bubble sort done
        if noswitch == array.len()-1 {
            break array;
        }
    };
    print!("The sorted array is: ");
    for x in sorted {
        print!("{} ", x);
    }
    println!("");
}
