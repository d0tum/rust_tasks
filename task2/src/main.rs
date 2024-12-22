struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut result = String::new();
        let mut rows = vec![String::new(); num_rows];
        let mut current_row = 0;
        let mut down = false;

        for ch in s.chars() {
            rows[current_row].push(ch);
            if current_row == 0 || current_row == num_rows - 1 {
                down = !down;
            }
            if down {
                current_row += 1;
            } else {
                current_row -= 1;
            }
        }

        for row in rows {
            result.push_str(&row);
        }
        result
    }
}

fn main() {
    let s1 = String::from("PAYPALISHIRING");
    let result = Solution::convert(s1, 4);
    println!("{}", result); 
}

