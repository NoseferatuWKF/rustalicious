mod binary_search;

use binary_search::sorted_binary_search;

fn main() {
    let haystack: Vec<usize> = vec![5, 13, 14, 21, 32, 44];
    let res = sorted_binary_search(&14, &haystack);
    println!("{res}");
}
