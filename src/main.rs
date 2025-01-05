pub mod fizzbuzz;
pub mod vector;

use rust_datastructure_algo::vector::{max_subarray, three_sum};
use vector::{array_product, longest_band, min_num_swap, mountain, pairs, rains, sub_array_sort, triplet};

fn main() {
    // test();
    // println!("{:?}", fizz_buzz(20));
    // pairs(-1);
    // triplet(18);
    three_sum(vec![-1, 0, 1, 2, -1, -4]);
    // mountain(vec![1, 2, 1, 3, 4, 5, 4, 3, 1, 5, 4, 3, 6]);
    // longest_band(vec![1, 9, 3, 0, 18, 5, 2, 4, 10, 7, 12, 6]);
    // rains(vec![0,1,0,2,1,0,1,3,2,1,2,1]);
    // sub_array_sort(vec![1, 2, 3, 4, 5, 8, 6, 7, 9, 10, 11]);
    // min_num_swap(&mut vec![2, 4, 5, 1, 3]);
    // array_product(vec![10, 12, -1, 4, -5]);
    // println!("max sum of sub array {}", max_subarray(vec![-1, 2, 3, 4, -2, 6, -8, 3]));
}
