pub trait Connection{

}

pub trait Repository{
    fn new(&self, connection: impl Connection) -> Self{
        
    }
}
