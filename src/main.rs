use std::ops::{Add, Sub};
use std::fmt::{Display, Formatter,Error};



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
//矩阵
#[derive(Debug)]
struct Matrix <T> where T:Add<T,Output=T>+Sub<T,Output = T>+Clone +Copy{
    value:Vec<Vector<T>>
}

impl <T> Display for Matrix<T> where T:Add<T,Output=T>+Sub<T,Output = T>+Clone +Copy{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let size = self.value.len();
        let mut format_str = String::new();
        write!(f,"Matrix {} x {}\n",size,self.value[0].size())
//        for i in 0..size{
//
//            write!()
//
//            format_str = format_str +"";
//
//        }
        //write!(f,"{}")
    }
}

impl <T> Matrix<T> where T:Add<T,Output=T>+Sub<T,Output = T>+Clone +Copy{

    fn new(list:Vec<Vector<T>>)->Matrix<T>{

        Matrix{
            value:list
        }


    }

//    //矩阵转置
//    fn transpose(&self)->Matrix<T>{
//
//    }

}
fn main(){

    let mat = Matrix::new(vec![
        Vector::new(vec![1,2,3,4]),
        Vector::new(vec![2,3,4,5])
    ]);

    let v1 = Vector::new(vec![1,2,3,4,655]);
    let v2 = Vector::new(vec![2,3,4,5,6]);

    println!("{}",mat);

}