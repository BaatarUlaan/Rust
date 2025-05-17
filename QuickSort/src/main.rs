fn main() {}

//======================quick sort
fn quick_sort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let mid = partition(arr); // partition 现在自己决定 pivot 的位置
        quick_sort(&mut arr[0..mid]);
        quick_sort(&mut arr[mid + 1..]);
    }
}

//choose the last number as pivot (within the current slice)
fn partition(arr: &mut [i32]) -> usize {
    let pivot_index = arr.len() - 1;
    let pivot = arr[pivot_index];
    let mut i = 0;
    let size = arr.len();

    for j in 0..size - 1 {
        if arr[j] > pivot {
            continue;
        } else {
            swap(arr, i, j);
            i += 1;
        }
    }
    swap(arr, i, pivot_index);
    i
}

fn swap(arr: &mut [i32], i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_test() {
        let mut a = vec![5, 6, 0, 1, 2];
        let expect =  2;
        assert_eq!(expect, partition(&mut a));
    }
    #[test]
    fn quick_sort_test() {
        let mut a = vec![5, 6, 0, 1, 2];
        let expect = vec![0, 1, 2, 5, 6];
        quick_sort(&mut a);
        assert_eq!(expect, a);
    }
}
