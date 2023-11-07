impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut i, mut j) : (usize, usize) = (0, 0);
        let mut sorted_nums1 = nums1.to_vec();
        let mut sorted_nums2 = nums2.to_vec();

        // Or we can change fn signature to:
        // intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32>
        sorted_nums1.sort();
        sorted_nums2.sort();

        let mut intersection: Vec<i32> = Vec::new();

        while i < sorted_nums1.len() && j < sorted_nums2.len() {
            if sorted_nums1[i] == sorted_nums2[j] {
                intersection.push(sorted_nums1[i]);
                i += 1;
                j += 1;
            } else if sorted_nums1[i] < sorted_nums2[j]{
                i += 1;
            } else {
                j += 1;
            }
        }

        intersection
    }
}