

fn main() {
    println!("Hello, world!");
}

fn merge_sort(arr:&[i32])->Box<[i32]>{
    if arr.len()==1{
        return arr.to_vec().into_boxed_slice();
    }
    
    let left;
    let right;
    
    let mid=arr.len() / 2;
    left=merge_sort(&arr[..mid]);
    right=merge_sort(&arr[mid..]);
    
    merge(&*left,&*right)
}

fn merge(array1_read_only:&[i32], array2_read_only:&[i32])-> Box<[i32]> {
    //NB: two arrays are presorted ascendingly
    let mut temp=Vec::new();
    
    let mut index1=0;
    let mut index2=0;

    loop {
        if index1==array1_read_only.len()&&index2==array2_read_only.len() {
            break;
        }
        else if index1==array1_read_only.len()&&index2<array2_read_only.len() { 
            for i in index2..array2_read_only.len() {
                temp.push(array2_read_only[i]);
            }
            break;
        }
        else if index2==array2_read_only.len()&&index1<array1_read_only.len() { 
            for i in index1..array1_read_only.len() {
                temp.push(array1_read_only[i]);
            }
            break;
        }
        
        if array1_read_only[index1]>array2_read_only[index2] { 
            temp.push(array2_read_only[index2]);
            index2+=1;
        }
        else { 
            temp.push(array1_read_only[index1]);
            index1+=1;
        }
    }
    
    temp.into_boxed_slice()
    
}

#[cfg(test)]
mod tests {
    use crate::{merge, merge_sort};

    #[test]
    fn merge_test(){
        let array1=[1,2,3,4,5];
        let array2=[0,5,7,9,10];
        let expected = [0,1,2,3,4,5,5,7,9,10];
        let result=merge(&array1,&array2);
        assert_eq!(&expected[..], &*result);
    }
    
    #[test]
    fn merge_sort_test(){
        let array1=[5,3,1,4,2];
        let expected1=[1,2,3,4,5];
        let result1=merge_sort(&array1);
        //println!("{:?}", result1);
        assert_eq!(&expected1[..], &*result1);
    }
}

