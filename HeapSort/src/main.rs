use std::cmp::max;

fn main() {

    //keep heapified
    let mut exa=vec![1,2,3,4];
    heapify(&mut exa);
    println!("{:?}",exa);

    let mut exa=vec![0];
    heapify(&mut exa);
    println!("{:?}",exa);

    //Sort
    let mut exa=vec![5,4,3,2,1];
    exa=heap_sort(exa);
    println!("{:?}",exa);
}
fn node_hipify(parent:usize,mut data:&mut [isize]){
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
    if largest==child1{
        node_hipify(child1,data);
        //println!("{:?}",data);
    }
    if largest==child2{
        node_hipify(child2,data);
        //println!("{:?}",data);
    }

}
fn heapify(a :&mut [isize]) {
    //the first parent node, from down to up
    let begin=((a.len() as f64-0.5)/2.0).floor() as usize;
    for i in (0..=begin).rev(){
        node_hipify(i,a);
    }

}

fn sink_top_after_heapify(a:&mut [isize]){
    let mut parent=0;
    let mut largest=parent;

    loop{
        let child1=2*parent+1;
        let child2=2*parent+2;

        if child1<a.len()&&a[child1]>a[largest]{
            largest=child1;
        }
        if child2<a.len()&&a[child2]>a[largest]{
            largest=child2;

        }

        if largest!=parent {
            a.swap(largest,parent);
            parent=largest;
        }
        else{break;}

    }


}
fn heap_sort(mut data :Vec<isize>) ->Vec<isize>{ //must not use heapify in loop
    heapify( &mut data); //from down to up     NlogN
    for i in (0..data.len()).rev(){    //NlogN
        //println!("Before swap{:?} i={}", data,i);
        data.swap(0, i);
        //println!("{:?} i={}", data,i);
        sink_top_after_heapify(&mut data[0..i]);//i must be excluded
        //println!("{:?}", data);
        //because already heapified, only need to sink vec[0]
    }
    data
}