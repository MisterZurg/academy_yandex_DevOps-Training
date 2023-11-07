use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set1: HashSet<i32> = HashSet::new();
        let mut set2: HashSet<i32> = HashSet::new();

        for &num in nums1.iter() {
            set1.insert(num);
        }

        for &num in nums2.iter() {
            set2.insert(num);
        }

        let mut difference: Vec<Vec<i32>> = vec![vec![]; 2];


        for num in nums1.iter() {
            if !set2.contains(&num) {
                difference[0].push(*num);
                set2.insert(*num);
            }
        }

        for num in nums2.iter() {
            if !set1.contains(&num) {
                difference[1].push(*num);
                set1.insert(*num);
            }
        }

        difference
    }
}