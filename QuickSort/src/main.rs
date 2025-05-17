use std::vec;

fn main() {}


//======================without recursion

fn quick_sort_without_recursion(arr: &mut [i32]) {
    let mut event_list:Vec<(usize,usize)>=Vec::new();
    event_list.push((0,arr.len()-1));
    
    while event_list.len() > 0 {
        let (x,y) = event_list.remove(0);
        let mid=partition(&mut arr[x..=y],y-x);//notice y-x SHOULD NOT USE Y
        
        if mid+1<=y { 
            event_list.push((mid+1,y)); 
        }
        if x>=mid-1{
            event_list.push((x,mid-1));
        }
        
    }
}
//======================quick sort
fn quick_sort( arr: &mut [i32])  {
    let mid;

    if arr.len()> 1 {
        let pivot = arr.len() - 1;
        //在二分的时候会产生空切片，如果这个时候把这行放在外面，那么就会形成0-1 attempt to subtract with overflow 错误
        mid = partition(arr, pivot);
        //println!("{:#?}pivot is {}", arr,pivot);
        quick_sort(&mut arr[0..mid]);
        //println!("{:#?}pivot is {} mid is {}", arr,pivot,mid);
        quick_sort(&mut arr[mid + 1..]);
    }


}

//choose the last number as pivot

fn partition(arr: &mut [i32], where_is_pivot: usize) ->  usize {
    let mut i = 0;
    let size = arr.len();
    let pivot = arr[where_is_pivot];
    swap(arr,where_is_pivot,arr.len()-1);
    for j in 0..size - 1 {
        if arr[j] > pivot {
            continue;
        } else {
            swap(arr, i, j);
            i += 1;
        }
    }
    swap(arr, i, where_is_pivot); //j==size
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
        assert_eq!(expect, partition(&mut a, 4));
    }
    #[test]
    fn quick_sort_test() {
        let mut a = vec![5, 6, 0, 1, 2];
        let expect = vec![0, 1, 2, 5, 6];
        quick_sort(&mut a);
        assert_eq!(expect, a);
    }
    #[test]
    fn quick_sort_recursion_test() {
        let mut a = vec![5, 6, 0, 1, 2];
        let expect = vec![0, 1, 2, 5, 6];
        quick_sort_without_recursion(&mut a);
        assert_eq!(expect, a);
    }
}
