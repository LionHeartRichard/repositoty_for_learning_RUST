use std::cmp::Ordering;

impl Solution {

    pub fn merge_top_solution(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        for i in (0..n+m).rev() {
            match (m, n) {
                // nums2 ended
                (_, 0) => {
                    break;
                }
                // nums1 ended
                (0, _) => {
                    nums1[i] = nums2[n-1];
                    n -= 1;
                }
                // both still going
                _ => {
                    match nums1.get(m-1).cmp(&nums2.get(n - 1)) {
                        // if nums1 is less, we import from nums2
                        Ordering::Less => {
                            nums1[i] = nums2[n - 1];
                            n -= 1;
                        },
                        // Otherwise we import from nums1
                        Ordering::Equal |
                        Ordering::Greater => {
                            nums1[i] = nums1[m - 1];
                            m -= 1;
                        }
                    }  
                }
            }
        }
    }


    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

        let m = m as usize;
        let n = n as usize;

        let mut i = 0 as usize;
        let mut j= 0 as usize;
        let mut k= 0 as usize;

        let vec = nums1.clone();

        while i<m && j<n {
            if vec[i]<nums2[j] {
                nums1[k]=vec[i];
                k+=1;
                i+=1;
            } else {
                nums1[k]=nums2[j];
                k+=1;
                j+=1;
            }
        }

        while i<m {
            nums1[k]=vec[i];
            k+=1;
            i+=1; 
        }

        while j<n {
            nums1[k]=nums2[j];
            k+=1;
            j+=1;
        }
    }
}


fn main() {
    let mut nums1:Vec<i32> = vec![2,4,6,8,10,12];
    let mut nums2:Vec<i32> = vec![1,3,5,7,9];

    


    print!("{:?}", nums1);
}
