impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        for ch in moves.chars() {
            match ch{
            'L' => x -= 1,
            'R' => x += 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => {}, // panic!(), production ready
            }
        }

        return x == 0 && y == 0;
    }
}