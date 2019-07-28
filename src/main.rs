use std::ops::Add;

#[derive(Debug)]
struct Vector<T> where T:Copy+Clone{
    value:Vec<T>
}

impl <T> Vector<T> where T:Add<T,Output=T>+Copy+Clone{
    fn new_from_list(list:&[T])->Vector<T>{
        Vector{
            value:Vec::from(list)
        }
    }
    fn new(list:Vec<T>)->Vector<T>{
        Vector{
            value:list
        }
    }
    fn size(&self)->u32{
        self.value.len() as u32
    }
    fn insert(&mut self,val:T)->&mut Vector<T>{
        self.value.push(val)
        &self
    }
}
//向量加法
impl <T> Add<Vector<T>> for Vector<T> where T:Add<T,Output=T>+Copy+Clone {
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size() , rhs.size());
        let mut r = Vec::new();
        let other_iterator = rhs.value.iter();

        let size = self.size();
        for i in 0..size{
            r.push(self.value[i as usize]+ rhs.value[i as usize])
        }
        Vector::new(r)
    }
}
fn main(){

    let v1 = Vector::new(vec![1,2,3,4,5]);
    let v2 = Vector::new(vec![2,3,4,5,6]);

    println!("{:?}",v1 + v2)

}