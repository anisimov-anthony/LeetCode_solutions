#![allow(dead_code)]
pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    let mut c = 0;
    let mut cntc = 0;
    let n = distance.len() as i32;

    let mut i = start;
    while i != destination {
        c += distance[i as usize];
        i = (i + 1) % n;
    }

    while i != start {
        cntc += distance[i as usize];
        i = (i + 1) % n;
    }

    c.min(cntc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_between_bus_stops_basic() {
        assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1), 1);
        assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2), 3);
        assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3), 4);
    }
}
