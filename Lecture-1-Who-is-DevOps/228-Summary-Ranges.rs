impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ranges: Vec<String> = Vec::new();
        if nums.len() == 0 {
            return ranges;
        }

        let mut l: i32 = nums[0];
        let mut r: i32 = nums[0];
        for (i, num) in nums[1..].iter().enumerate()  {
            if r + 1 == *num {
                r = *num;
                continue;
            }

            if l == r {
                ranges.push(format!("{}", l));
            } else {
                ranges.push(format!("{}->{}", l, r));
            }
            l = *num;
            r = *num;
        }

        if l == r {
            ranges.push(format!("{}", l));
        } else {
            ranges.push(format!("{}->{}", l, r));
        }

        return ranges;
    }
}