/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    // 外层循环：重复 len 次，或者直到没有交换发生为止
    for i in 0..len {
        // 内层循环：比较相邻元素，并进行交换
        // 注意：每完成一次外层循环，最后 i 个元素已排序，可略过
        for j in 0..(len - 1 - i) {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
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