// use sqlx::{Pool, Postgres};

// pub trait RepositoryTrait {
//     type Row;

//     // fn new(db:&Pool<Postgres>);
    
//     async fn rows() -> Result<Self::Row, sqlx::Error>;
//     async fn create<T>(&self, item: T) -> Result<Self::Row, sqlx::Error> where T:NTR;
//     async fn detail() -> Result<Self::Row, sqlx::Error>;
//     async fn update(&self) -> Result<Self::Row, sqlx::Error>;
//     async fn delete(&self) -> Result<Self::Row, sqlx::Error>;
// }
