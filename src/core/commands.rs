pub enum Commands {
    Create(String),
    Remove(u32),
    Update(u32, String),
    List,
}
