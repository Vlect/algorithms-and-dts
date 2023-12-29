/*
PROBLEM STATEMENT:
- Given two arrays nums1 and nums2 of size `m` and `n` respectively, return
*the median* of the two sorted arrays.

The overall run time complexity should be 0(log (m + n))

Example 1:
- Input: nums1 = [1,3], nums2 = [2]
- Output: 2.00000
- Explanation: merged array = [1,2,3] and median is 2.

Example 2:
- Input: nums1 = [1,2], nums2 = [3,4]
- Output: 2.50000
- Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
*/
pub mod solution {
    pub struct Solution {
        p1: usize,
        p2: usize,
    }

    impl Solution {
        pub fn new() -> Self {
            Solution { p1: 0, p2: 0 }
        }

        // Get the smaller value between nums1[p1] and nums2[p2] and move the pointer forward.
        fn get_min(&mut self, nums1: &[i32], nums2: &[i32]) -> i32 {
            if self.p1 < nums1.len() && self.p2 < nums2.len() {
                if nums1[self.p1] < nums2[self.p2] {
                    let val = nums1[self.p1];
                    self.p1 += 1;
                    println!("P1/P2 are both less than nums1/2.len(): {}", self.p1);
                    return val;
                } else {
                    let val = nums2[self.p2];
                    self.p2 += 1;
                    return val;
                }
            } else if self.p1 < nums1.len() {
                println!("P1 is less than nums1.len(): {}", self.p1);
                let val = nums1[self.p1];
                self.p1 += 1;
                println!("WHAT: {}", val);
                return val;
            } else if self.p2 < nums2.len() {
                println!("P2 is less than nums2.len(): {}", self.p1);
                let val = nums2[self.p2];
                self.p2 += 1;
                return val;
            }
            -1 // Assuming this case doesn't happen with valid input
        }

        pub fn find_median_sorted_arrays(&mut self, nums1: &[i32], nums2: &[i32]) -> f64 {
            let m = nums1.len();
            let n = nums2.len();

            if (m + n) % 2 == 0 {
                for _ in 0..(m + n) / 2 - 1 {
                    let _ = self.get_min(nums1, nums2);
                }
                let n1 = self.get_min(nums1, nums2);
                let n2 = self.get_min(nums1, nums2);
                println!("{} {}", n1, n2);
                (n1 as f64 + n2 as f64) / 2.0
            } else {
                for _ in 0..(m + n) / 2 {
                    let _ = self.get_min(nums1, nums2);
                }
                self.get_min(nums1, nums2) as f64
            }
        }
    }
}

pub mod tests {
    use crate::hard::median_of_two_sorted_arrays::solution::Solution;

    //#[test]
    //fn test_1() {
    //    let mut sol = Solution::new();
    //    let nums1 = vec![1, 3];
    //    let nums2 = vec![2];
    //    assert_eq!(sol.find_median_sorted_arrays(&nums1, &nums2), 2.0);
    //}

    //#[test]
    //fn test_2() {
    //    let mut sol = Solution::new();
    //    let nums1 = vec![1, 2];
    //    let nums2 = vec![3, 4];
    //    assert_eq!(sol.find_median_sorted_arrays(&nums1, &nums2), 2.5);
    //}

    //#[test]
    //fn test_3() {
    //    let mut sol = Solution::new();
    //    let nums1 = vec![0, 0];
    //    let nums2 = vec![0, 0];
    //    assert_eq!(sol.find_median_sorted_arrays(&nums1, &nums2), 0.0);
    //}

    //#[test]
    //fn test_4() {
    //    let mut sol = Solution::new();
    //    let nums1 = vec![];
    //    let nums2 = vec![1];
    //    assert_eq!(sol.find_median_sorted_arrays(&nums1, &nums2), 1.0);
    //}

    //#[test]
    //fn test_5() {
    //    let mut sol = Solution::new();
    //    let nums1 = vec![2];
    //    let nums2 = vec![];
    //    assert_eq!(sol.find_median_sorted_arrays(&nums1, &nums2), 2.0);
    //}

    #[test]
    fn test_6() {
        let mut sol = Solution::new();
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        assert_eq!(sol.find_median_sorted_arrays(&nums1, &nums2), 3.5);
    }
}
