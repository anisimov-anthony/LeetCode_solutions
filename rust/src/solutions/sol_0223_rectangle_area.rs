#[allow(dead_code)]
pub fn compute_area(
    ax1: i32,
    ay1: i32,
    ax2: i32,
    ay2: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
) -> i32 {
    let s1 = (ax2 - ax1) * (ay2 - ay1);
    let s2 = (bx2 - bx1) * (by2 - by1);

    let lbound = ax1.max(bx1);
    let rbound = ax2.min(bx2);
    let horiz_diff = rbound - lbound;

    let bbound = ay1.max(by1);
    let tbound = ay2.min(by2);
    let vert_diff = tbound - bbound;

    let overlap = if horiz_diff > 0 && vert_diff > 0 {
        horiz_diff * vert_diff
    } else {
        0
    };

    s1 + s2 - overlap
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute_area_basic() {
        assert_eq!(compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
        assert_eq!(compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
    }
}
