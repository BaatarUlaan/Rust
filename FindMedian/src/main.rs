

fn main() {
    println!("Hello, world!");
}

fn find_medan_among_5(arr: & [i32])->i32 {
    //find 2 pairs of (,)
    let (b, a) = cmp_1st_and_2nd_return_index_in_oder(&arr[0..=1], 0);
    let (d, c) = cmp_1st_and_2nd_return_index_in_oder(&arr[2..=3], 2);
    
    let (temp_array_3,remain) = 
        if arr[a] >= arr[c] {
            ([arr[d],arr[c],arr[a]],arr[b])
        }
    else {
        ([arr[b],arr[a],arr[c]],arr[d])
    };
    
    let min_of_remain_2;
    let max_of_remain_2;
    if  remain<=arr[4]{
        min_of_remain_2=remain;
        max_of_remain_2=arr[4];
    }
    else { min_of_remain_2=arr[4];
        max_of_remain_2=remain; }
    
    
    if min_of_remain_2>temp_array_3[1] { 
        min_of_remain_2
    }
    else{
        if max_of_remain_2>temp_array_3[1] { temp_array_3[1] }
        else { max_of_remain_2 }
    }
    
}


fn cmp_1st_and_2nd_return_index_in_oder(slice: &[i32], start_from: usize) -> (usize, usize) {
    //(b,a) means a>=b
    if slice.len() < 2 {
        (0, 0)
    } else if slice[0] >= slice[1] {
        (1 + start_from, 0 + start_from)
    } else if slice[0] <= slice[1] {
        (0 + start_from, 1 + start_from)
    } else {
        //unexpected error happens
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmp_1st_and_2nd_return_index_in_oder, find_medan_among_5};

    #[test]
    fn cmp_1st_and_2nd_return_index_in_oder_test() {
        let a = [1, 2, 3, 4, 5];
        let expect = (3, 4);
        assert_eq!(cmp_1st_and_2nd_return_index_in_oder(&a, 3), expect);
    }

    #[test]
    fn find_medan_among_5_test() {
        let a = [8,6,7,4,1];
        let expect=6;
        assert_eq!(find_medan_among_5(&a), expect);
    }
}
