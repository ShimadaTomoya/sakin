pub mod invert_index {
    pub struct Token {
        id: u64,
        body: String
    }

    pub struct Document {
        id: u64,
        body: String
    }

    pub fn build_token(id: u64, body: String) -> Token {
        Token {
            id,
            body
        }
    }

    pub fn build_document(id: u64, body: String) -> Document {
        Document {
            id,
            body
        }
    }
}

fn main() {
    let _token = invert_index::build_token(0, "body".to_string());
    let _document = invert_index::build_document(0, "body".to_string());
}
