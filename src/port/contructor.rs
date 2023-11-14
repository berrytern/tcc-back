

pub trait Contructor{
    fn new<T>(&self, dependency: &impl T) -> Self;    
}