# CodeWars-You-re-a-square--7-kyu---Passed-
A square of squares
You like building blocks. You especially like building blocks that are squares. And what you even like more, is to arrange them into a square of square building blocks!

However, sometimes, you can't arrange them into a square. Instead, you end up with an ordinary rectangle! Those blasted things! If you just had a way to know, whether you're currently working in vain… Wait! That's it! You just have to check if your number of building blocks is a perfect square.

Task
Given an integral number, determine if it's a square number:

In mathematics, a square number or perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself.

The tests will always use some integral number, so don't worry about that in dynamic typed languages.

Examples
-1  =>  false
 0  =>  true
 3  =>  false
 4  =>  true
25  =>  true
26  =>  false


TEST CASES:
#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use super::is_square;
    use rand::seq::SliceRandom;

    #[test]
    fn fixed_tests() {
        assert_eq!(is_square(-1), false, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(0), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(3), false, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(4), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(25), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(26), false, "\nYour answer (left) is not the expected answer (right).");
    }
    
    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        let mut arr = Vec::new();
        for _ in 0..100 {
            arr.push((rng.gen_range(0..=67108864_i64).pow(2), true))
            }
        for _ in 0..100 {
            if rng.gen_bool(1.0/4.0) {
                arr.push((rng.gen_range(- (1 << 53) .. 0), false))
            } else {
                let n = rng.gen_range(0 .. (1 << 53));
                // reference solution updated following Unnamed's fork: https://www.codewars.com/kumite/62b0aacab97d820023a73bcb?sel=62b0aacab97d820023a73bcb
                let expected = ((n as f64).sqrt() as i64).pow(2) == n;
                arr.push((n,expected))
            }
        }
        arr.shuffle(&mut rng);
        for (n, expected) in arr.iter() {
            assert_eq!(is_square(*n), *expected, "\nWith n = {}, your answer (left) is not the expected answer (right).", n);
        }
    }
}
