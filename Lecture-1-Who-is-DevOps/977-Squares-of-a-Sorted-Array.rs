impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 {
            return vec![i32::pow(nums[0], 2)];
        }

        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;
        let mut squares = vec![0; nums.len()];

        for i in (0..nums.len()).rev() {
            if nums[l].abs() > nums[r].abs() {
                squares[i] = i32::pow(nums[l], 2);
                l += 1;
            } else {
                squares[i] = i32::pow(nums[r], 2);
                r -= 1;
            }
        }

        return squares;
    }
}

// Менее изящное решение
// impl Solution {
//     pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
//         if nums.len() == 1 {
//             return vec![i32::pow(nums[0], 2)];
//         }
//
//         let mut l: usize = 0;
//         let mut r: usize = nums.len() - 1;
//         let mut cur: usize = nums.len() - 1;
//         let mut squares = vec![0; nums.len()];
//
//         let mut cmp_left = i32::pow(nums[l], 2);
//         let mut cmp_rght = i32::pow(nums[r], 2);
//
//         while l != r {
//             if cmp_left > cmp_rght {
//                 squares[cur] = cmp_left;
//                 l += 1;
//                 cmp_left = i32::pow(nums[l], 2)
//             } else {
//                 squares[cur] = cmp_rght;
//                 r -= 1;
//                 cmp_rght = i32::pow(nums[r], 2);
//             }
//             cur -= 1;
//         }
//
//         squares[0] = cmp_rght;
//         return squares;
//     }
// }