pub struct EmbedRoll {
    pub name: String,
    pub serie: String,
    pub url: String,
}

impl EmbedRoll {
    pub fn new(name: String, serie: String, url: String) -> Self {
        Self { name, serie, url }
    }
}
