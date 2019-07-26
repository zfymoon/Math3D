use std::ops::Add;


#[derive(Debug,Clone)]
struct Vector2D<T>(T,T) where T:Clone;

#[derive(Debug,Clone)]
struct Vector3D<T>(T,T,T) where T:Clone;

#[derive(Debug,Clone)]
struct Vector4D<T>(T,T,T,T) where T:Clone;

impl <T> Add<Vector2D<T>> for Vector2D<T> where T:Clone+Add<T,Output = T>{
    type Output = Vector2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D(self.0 + rhs.0,self.1 + rhs.1)
    }
}

fn main(){
    println!("{:?}",Vector2D(1,2) + Vector2D(2,3))
}