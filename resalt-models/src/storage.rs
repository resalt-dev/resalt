pub trait StorageImpl: Send + Sync {
    fn clone(&self) -> Box<dyn StorageImpl>;

    fn clone_self(&self) -> Box<dyn StorageImpl> {
        self.clone()
    }

    // Extremely generic KV operations

    fn get(&self, key: &str) -> Result<Option<String>, String>;

    fn set(&self, key: &str, value: &str) -> Result<(), String>;

    fn del(&self, key: &str) -> Result<(), String>;

    fn keys(&self, prefix: &str) -> Result<Vec<String>, String>;
}

impl Clone for Box<dyn StorageImpl> {
    fn clone(&self) -> Box<dyn StorageImpl> {
        self.clone_self()
    }
}
