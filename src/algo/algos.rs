use std::ops::Sub;

pub fn max_diff<T: Ord + Sub<Output=T> + Copy>(ints: &[T], min: T) -> T {
    //tail-recursive
    fn find_max_diff<T: Ord + Sub<Output=T> + Copy>(max: &T, diff: T, rest: &[T]) -> T {
        if rest.is_empty() {
            return diff;
        }
        if rest[0] > *max {
            if rest.len() > 1 {
                let m = &rest[0];
                find_max_diff(m, diff, &rest[1..])
            } else {
                diff
            }
        } else {
            find_max_diff(
                max,
                Ord::max(diff, max.sub(rest[0])),
                &rest[1..]
            )
        }
    }
    if ints.len() <= 1 {
        return min;
    }
    let mut reverse = ints.to_vec();
    reverse.reverse();
    find_max_diff(&min, reverse[0], &reverse[1..])
}

#[test]
fn test_find_max_diff_i32() {
    assert_eq!(
        max_diff(&[7, 1, 5, 3, 6, 4], i32::MIN),
        5
    )
}
