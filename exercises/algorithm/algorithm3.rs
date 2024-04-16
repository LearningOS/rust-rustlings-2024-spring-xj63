/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord + std::fmt::Debug>(array: &mut [T]) {
    let (mid, arrayn) = match array.split_first_mut() {
        None => return,
        Some((mid, arrayn)) if arrayn.is_empty() => return,
        Some((mid, arrayn)) => (mid, arrayn),
    };
    let (mut l, mut r): (usize, usize) = (0, arrayn.len() - 1);

    while l < r {
        if arrayn[l] > *mid {
            arrayn.swap(l, r);
            r -= 1;
        } else {
            l += 1;
        }
    }

    let (mut l, mut r): (usize, usize) = (l + 1, r + 1);
    if array[r] > array[0] {
        array.swap(r - 1, 0);
        r -= 1;
    } else {
        array.swap(l, 0);
        l += 1;
    }

    sort(&mut array[..(l - 1)]);
    sort(&mut array[(r + 1)..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        let mut vec_sort = vec.clone();
        vec_sort.sort();
        sort(&mut vec);
        assert_eq!(vec, vec_sort);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }

    // my test
    #[test]
    fn test_sort_4() {
        let mut vec = vec![5, 4, 3, 2, 1];
        sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_sort_5() {
        let mut vec = vec![100, 200, 300, 400, 500];
        sort(&mut vec);
        assert_eq!(vec, vec![100, 200, 300, 400, 500]);
    }
    #[test]
    fn test_sort_6() {
        let mut vec = vec![56, 78, 34, 23, 89, 45];
        sort(&mut vec);
        assert_eq!(vec, vec![23, 34, 45, 56, 78, 89]);
    }
    #[test]
    fn test_sort_7() {
        let mut vec = vec![90, 80, 70, 60, 50, 40, 30, 20, 10];
        sort(&mut vec);
        assert_eq!(vec, vec![10, 20, 30, 40, 50, 60, 70, 80, 90]);
    }
    #[test]
    fn test_sort_8() {
        let mut vec = vec![5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5];
        let mut vec_clone = vec.clone();
        sort(&mut vec);
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }
    #[test]
    fn test_sort_9() {
        let mut vec = vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];
        let mut vec_clone = vec.clone();
        sort(&mut vec);
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }
    #[test]
    fn test_sort_10() {
        let mut vec = vec![56, 78, 34, 23, 89, 45, 67, 12, 39, 81, 27, 60];
        let mut vec_clone = vec.clone();
        sort(&mut vec);
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }
    #[test]
    fn test_sort_11() {
        let mut vec = vec![
            90, 80, 70, 60, 50, 40, 30, 20, 10, 0, -10, -20, -30, -40, -50, -60, -70, -80, -90,
        ];
        let mut vec_clone = vec.clone();
        sort(&mut vec);
        vec_clone.sort();
        assert_eq!(vec, vec_clone);
    }

    // how to use rand?
    // #[test]
    // fn test_sort_rand() {
    //     let mut rng = rand::thread_rng();
    //     let mut vec: Vec<i32> = (0..100).map(|_| rng.gen_range(0, 100)).collect();
    //     let mut vec_clone = vec.clone();
    //     sort(&mut vec);
    //     vec_clone.sort();
    //     assert_eq!(vec, vec_clone);
    // }
}
