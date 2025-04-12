/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn partition<T: Ord>(array: &mut [T], low: isize, high: isize) -> isize {
    let piovt = high as usize;
    let mut i = low - 1;
    for j in low..high {
        if array[j as usize] <= array[piovt] {
            i += 1;
            array.swap(i as usize, j as usize);
        }
    }
    array.swap((i + 1) as usize, piovt);
    i + 1
}

fn quick_sort<T: Ord>(array: &mut [T], low: isize, high: isize) {
    if low < high {
        let piovt = partition(array, low, high);
        quick_sort(array, low, piovt - 1);
        quick_sort(array, piovt + 1, high);
    }
}

fn sort<T: Ord>(array: &mut [T]) {
    quick_sort(array, 0, array.len() as isize - 1);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
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
}
