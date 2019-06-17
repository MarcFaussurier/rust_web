#[derive(Clone)]
pub struct MemoryCell {
    pub text: String,
    pub cell_type: String,
    pub id: String,
    pub scopes: Vec<i8>
}