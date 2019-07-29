use std::ops::{Add, Sub};

#[derive(Debug)]
struct Vector<T> where T:Copy+Clone{
    value:Vec<T>
}

impl <T> Vector<T> where T:Add<T,Output=T>+Sub<T,Output = T>+Copy+Clone{
    fn new(list:Vec<T>)->Vector<T>{
        Vector{
            value:list
        }
    }
    fn size(&self)->u32{
        self.value.len() as u32
    }

}
//向量加法
impl <T> Add<Vector<T>> for Vector<T> where T:Add<T,Output=T>+Sub<T,Output = T>+Clone +Copy {
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size() , rhs.size());
        let mut r = Vec::new();

        let size = self.size();
        for i in 0..size{
            r.push(self.value[i as usize]+ rhs.value[i as usize])
        }
        Vector::new(r)
    }
}
//向量减法
impl <T> Sub<Vector<T>> for Vector<T> where T:Add<T,Output=T>+Sub<T,Output = T>+Clone +Copy {
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size(),rhs.size());
        let mut result = Vec::new();
        let size = self.size();
        for i in 0..size{
            result.push(self.value[i as usize] - rhs.value[i as usize]);
        }
        Vector::new(result)
    }
}
fn main(){

    let v1 = Vector::new(vec![1,2,3,4,655]);
    let v2 = Vector::new(vec![2,3,4,5,6]);

    println!("{:?}",v1 - v2);

}