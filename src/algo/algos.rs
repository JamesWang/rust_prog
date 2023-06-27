use std::collections::HashMap;
use std::iter::Enumerate;
use std::ops::{Index, Sub};
use num::Num;
use serde::de::Unexpected::Map;

pub fn max_diff<T: PartialOrd + Sub<Output=T> + Copy>(ints: &[T], min: T) -> T {
    //tail-recursive
    fn find_max_diff<T: PartialOrd + Sub<Output=T> + Copy>(max: &T, diff: T, rest: &[T]) -> T {
        if rest.is_empty() {
            return diff;
        }
        if rest[0] > *max {
            if rest.len() > 1 {
                find_max_diff(&rest[0], diff, &rest[1..])
            } else { diff }
        } else {
            let n_diff = max.sub(rest[0]);
            find_max_diff(
                max,
                if PartialOrd::ge(&diff, &n_diff) { diff } else { n_diff },
                &rest[1..],
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

#[cfg(test)]
mod find_tests {
    use super::*;

    #[test]
    fn test_find_max_diff_i32() {
        assert_eq!(
            max_diff(&[7, 1, 5, 3, 6, 4], i32::MIN),
            5
        );
    }

    #[test]
    fn test_find_max_diff_float() {
        let arr: [f32; 6] = [7.4, 1.2, 5.8, 3.1, 6.0, 4.4];
        assert_eq!(
            max_diff(&arr, f32::MIN),
            4.8
        )
    }

    #[test]
    fn test_find_max_diff_info() {
        assert_eq!(
            max_diff_info(&[7, 1, 5, 3, 6, 4], i32::MIN),
            (5, (1, 6))
        );
    }

    #[test]
    fn test_find_max_diff2() {
        assert_eq!(
            max_diff2(&[7, 1, 5, 3, 6, 4], i32::MIN),
            5
        );
    }
}
type Pair<T> = (T, T);

pub fn max_diff_info<T: PartialOrd + Sub<Output=T> + Copy>(ints: &[T], min: T) -> (T, Pair<T>) {
    //tail-recursive
    fn find_max_diff<T: PartialOrd + Sub<Output=T> + Copy>(max: &T, diff: T, rest: &[T], pair: Pair<T>) -> (T, Pair<T>) {
        if rest.is_empty() {
            return (diff, pair);
        }
        if rest[0] > *max {
            if rest.len() > 1 {
                find_max_diff(&rest[0], diff, &rest[1..], pair)
            } else { (diff, pair) }
        } else {
            let n_diff = max.sub(rest[0]);
            let c_diff = if PartialOrd::ge(&diff, &n_diff) { diff } else { n_diff };
            find_max_diff(
                max,
                c_diff,
                &rest[1..],
                (rest[0], *max)
            )
        }
    }
    if ints.len() <= 1 {
        return (min, (min, min));
    }
    let mut reverse = ints.to_vec();
    reverse.reverse();
    find_max_diff(&min, reverse[0], &reverse[1..], (min, min))
}

fn max_diff2<T: PartialOrd + Sub<Output=T> + Copy>(ints: &[T], min: T) -> T {
    return max_diff_info(ints, min).0
}

fn two_int_sum_in_array(nums: &[i32], target: i32) -> Vec<usize> {
    let mut id_map: HashMap<i32,usize> = HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
        let diff = target - nums[idx];
        if id_map.get(&diff).is_some() {
            return vec![*id_map.get(&diff).unwrap(), idx];
        }
        id_map.insert(*num, idx);
    }
    return vec![0; 0];
}

#[test]
fn test_sum_in_array_with_target(){
    let arr: Vec<i32> = vec![0, 1, 2, 7, 11, 15];
    let result = two_int_sum_in_array(&arr, 9);
    assert_eq!(result, vec![2,3])
}