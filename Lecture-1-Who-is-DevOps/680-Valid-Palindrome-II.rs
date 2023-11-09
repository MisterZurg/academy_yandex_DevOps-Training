impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();

        let mut l = 0;
        let mut r = s.len() - 1;

        while l < r {
            if s[l] != s[r] {
                return Self::is_palindrome(l, r - 1, &s) || Self::is_palindrome(l + 1, r, &s);
            }
            l += 1;
            r -=1 ;
        }
        return true;
    }
    // NB: an array’s type using square brackets with the type
    fn is_palindrome(mut l: usize, mut r: usize, s: &[u8]) -> bool {
        while l < r {
            if &s[l] != &s[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        return true;
    }
}