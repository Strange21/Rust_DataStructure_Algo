pub fn test(){
    let mut arr:Vec<i32> = vec![0;10];
    arr.push(14);
    arr.pop();
    println!("sizeof vector {}", arr.capacity());
    for i in arr{
        println!("{}", i);
    }
}

pub fn pairs(num: i32) {
    // given a arry find the distinct pair which sums to a given number with time complexity O(n).
    let arr: Vec<i32> = vec![10, 2, 3, 5, -6, 9, 11];
    let mut my_set: HashSet<i32> = HashSet::new();
    for i in arr {
        let check = num - i;
        if my_set.contains(&check) {
            println!("pair found {}, {}", i, check);
        } else {
            my_set.insert(i);
        }
    }
}

pub fn triplet(num: i32) {
    // given a arry find the distinct triplet which sums to a given number, the triplet should be in sorted order.
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 15];
    let mut len = arr.capacity() - 1;
    for (i, v) in arr.iter().enumerate() {
        let check = num - v;
        let mut left = i + 1;
        let mut right = len;
        // println!("i = {}, check {}", arr[i], check);
        // println!("arr[left] = {}, arr[right] = {}", arr[left], arr[right]);
        while (left < right) {
            let sum = arr[left] + arr[right];
            if sum == check {
                println!("triplet found {}, {}, {}", v, arr[left], arr[right]);
                left += 1;
                right -= 1;
                break;
            } else if sum < check {
                left += 1;
            } else {
                right -= 1;
            }
        }
        // if i == arr.len() -3{
        //     break;
        // }
    }
}

pub fn mountain(arr: Vec<i32>) {
    let mut largest = 0;
    let mut i =1;
    while (i < arr.len() -1){
        let mut count = 1;
        if arr[i] > arr[i-1] && arr[i ]> arr[i+1]{
            let mut j = i;
            while arr[j] > arr[j -1] && j >1 {
                j -= 1;
                count +=1;
            }

            while arr[i] > arr[i + 1] && i < (arr.len()-1) {
                i += 1;
                count +=1;
            }
            println!("largest {}, count {}", largest, count);
            largest = max(largest, count);
            
        }
        else{
            i+=1;
        }
    }
    println!("largest mountain {}", largest);
}

pub fn longest_band(arr: Vec<i32>){
    let mut set:HashSet<i32> = HashSet::new();
    for i in arr.clone(){
        set.insert(i);
    }
    let mut largest_band = 0;
    let mut i =0;
    while i < arr.len(){
        
        let mut cnt = 1;
        // check if i can be the starting point of band?
        if set.contains(&(arr[i]-1)){
            i += 1; 
        } else if set.contains(&(arr[i]+1)){   // arr[i] can be the starting point 
            cnt += 1;
            let mut value = arr[i] + 1;
            while set.contains(&(value +1)){   // check other occurance of band
                cnt += 1;
                value += 1;
            }
            println!("band lenght {}", cnt);
            i += 1;
        } else{
            //  band lenght is 1
            i += 1;
        }
        largest_band = max(largest_band, cnt);
    }
    println!("largest band is of size {}", largest_band);
}

pub fn rains(arr: Vec<u32>){
    let mut len = arr.len(); // get the lenght   
    let mut left: Vec<u32> = vec![0; len];  // create left array
    let mut right: Vec<u32> = vec![0; len];  // create right arry
    
    // initialize left arry
    left[0] = arr[0];
    for i in 1..len{
        left[i] = left[i-1].max(arr[i]);
    }
    // initialise right arry
    right[len-1] = arr[len -1];
    for i in (0..len-1).rev() {
        right[i] = right[i + 1].max(arr[i]);
    }
    // compute the water storage
    let mut stored_water = 0;
    for i in (0..len-1){
        let water_level = left[i].min(right[i]);
        stored_water += water_level - arr[i];
    }
    println!("left {:?}", left);
    println!("right {:?}", right);
    println!("arr {:?}", arr);
    println!("Stored water {}", stored_water);
}

fn isOutOfOrder(arr: &Vec<i32>, i: usize) -> bool{
    if i== 0{
        return arr[i] > arr[i + 1];
    }
    if i == arr.len() -1 {
        return arr[i] < arr[i -1];
    }
    else{
        return arr[i] < arr[i-1] || arr[i] > arr[i+1];
    }
}

pub fn sub_array_sort(arr: Vec<i32>){
    let mut i =1;
    let len = arr.len();
    let mut smallest = 65535;
    let mut largest  = 0;
    //  check which element is out of order 
    //  an element is put of order if it is less than previous or it is less than next element
    // 
    while i < (len -1) {
        if isOutOfOrder(&arr, i){
            smallest = std::cmp::min(smallest, arr[i]);
            largest = max(largest, arr[i]);
        }
        i+=1;
    }
    println!("smallest {}, largest {}", smallest, largest);
    //  check where does largest & smallest fits
    let mut left = 0;
    while (arr[left] < smallest){
        left += 1;
    }

    let mut right = arr.len()-1;
    while (arr[right] > largest){
        right -= 1;
    }

    println!("sub array idx {}, {}", smallest, largest);
}

pub fn min_num_swap(arr: &mut Vec<i32>) {
    let mut ans  =0;
    
    let mut pair : Vec<(i32, i32)> = Vec::new();
    for (i, v) in arr.iter().enumerate(){
        pair.push((*v, i as i32));
    }
    pair.sort();
    let len = arr.len();
    let mut visited: Vec<bool> = vec![false; len];

    let mut i =0;
    
    while i < len{
        let old_index = pair[i].1;
        if visited[i] == true || old_index == i as i32{
            i+=1;
            continue;
        }

        let mut node = i;
        let mut cycle = 0;
        while(!visited[node]){
            visited[node] = true;
            node = pair[node].1 as usize;
            cycle += 1;
        }
        i += 1;
        ans += cycle -1;
        
    }
    println!("min swaps {}", ans);

}

pub fn array_product(arr: Vec<i32>){
    let mut result : Vec<i32> = vec![1; arr.len()];
    let mut prefix = 1;
    let mut suffix =1;

    for i in 0..arr.len(){
        result[i] = prefix;
        prefix *= arr[i];
    }

    for i in (0..arr.len()).rev() {
        result[i] *= suffix;
        suffix *= arr[i];
    }

    println!("Result : {:?}", result);
}