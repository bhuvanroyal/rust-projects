fn main() {
    let mut  v=vec![3,1,8,4,9,10,6,15,7,2,5,5];

    for i in 0..v.len(){
        let mut min=i;
        for j in i+1..v.len(){
            if v[min]>v[j]{
                min=j;
            }
        }
        if i!=min {
            let temp=v[i];
            v[i]=v[min];
            v[min]=temp;
        }
    }

    println!("median is {:?}",v.get((v.len()-1)/2));
    println!("{v:?}");
}
