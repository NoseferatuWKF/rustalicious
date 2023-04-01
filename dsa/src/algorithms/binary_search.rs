pub fn sorted_binary_search(needle: &usize, haystack: &Vec<usize>) -> bool {

    let mut low: usize = 0;
    let mut high: usize = haystack.len();

    while low < high {

        let middle = &low + (&high - &low) / 2;

        if haystack[middle].eq(needle) {
            return true;
        } else if &haystack[middle] > needle {
            high = middle;
        } else {
            low = middle + 1;
        }

    }
    false
}

#[cfg(test)]
mod tests {

    use super::sorted_binary_search;

    #[test]
    fn test_sorted_binary_search_exists() {
        let haystack = vec![1, 2, 4, 6, 7, 8];
        let exists = sorted_binary_search(&2, &haystack);
        assert_eq!(exists, true);
    }

    #[test]
    fn test_sorted_binary_search_not_exist() {
        let haystack = vec![1, 2, 4, 6, 7, 8];
        let not_exists = sorted_binary_search(&3, &haystack);
        assert_eq!(not_exists, false);
    }

}
