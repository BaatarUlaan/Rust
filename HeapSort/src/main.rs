fn main() {
    let mut exa=vec![1,2,3,4,5];
    exa=heapify(exa);
    println!("{:?}",exa);

    let mut exa=vec![0];
    exa=heapify(exa);
    println!("{:?}",exa);

}
fn node_hipify(parent:usize,mut data:Vec<isize>)->Vec<isize>{
    let child1=2*parent+1;
    let child2=2*parent+2;
    let mut largest=parent;

    if child1<data.len()&&data[child1]>data[largest]{
        largest=child1;
    }
    if child2<data.len()&&data[child2]>data[largest]{
        largest=child2;
    }

    if largest!=parent {
        data.swap(largest,parent);
    }


    //trace the sunk "parent" node
    if largest==child1{data=node_hipify(child1,data);}
    if largest==child2{data=node_hipify(child2,data);}

    data
}
fn heapify(mut a :Vec<isize>) ->Vec<isize>{
    //the first parent node, from down to up
    let begin=((a.len() as f64-0.5)/2.0).floor() as usize;
    for i in (0..=begin).rev(){
        a=node_hipify(i,a);
    }
    a
}



