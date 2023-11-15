
pub trait Contructor{
    type Dependency;
    fn new(&self, service: Self::Dependency  ) -> Self;    
}