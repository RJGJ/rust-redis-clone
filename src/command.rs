pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Del { key: String },
    Ping,
    // Exists { key: String },
}
