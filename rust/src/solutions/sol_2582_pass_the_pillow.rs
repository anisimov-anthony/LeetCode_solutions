#[allow(dead_code)]
pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let cycles = time / (n - 1);
    let added_steps = time % (n - 1);
    if cycles % 2 == 0 {
        return added_steps + 1;
    } else {
        return n - added_steps;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pass_the_pillow_basic() {
        assert_eq!(pass_the_pillow(4, 5), 2);
        assert_eq!(pass_the_pillow(3, 2), 3);
    }
}
