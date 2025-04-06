pub trait RepositoryTrait {
    type Row;
    
    async fn rows() -> Result<Self::Row, sqlx::Error>;
    async fn create(&self) -> Result<Self::Row, sqlx::Error>;
    async fn detail() -> Result<Self::Row, sqlx::Error>;
    async fn update(&self) -> Result<Self::Row, sqlx::Error>;
    async fn delete(&self) -> Result<Self::Row, sqlx::Error>;
}
