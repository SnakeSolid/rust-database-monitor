#[derive(Debug)]
pub struct Query {
    tokens: Vec<String>,
}

impl Query {
    pub fn new(words: &[&str]) -> Query {
        let mut tokens = Vec::new();

        for word in words {
            let word = word.to_lowercase();

            tokens.extend(word.split(|c| " _-".contains(c)).map(|token| token.into()));
        }

        Query { tokens }
    }

    pub fn tokens(&self) -> &Vec<String> {
        &self.tokens
    }
}

impl From<String> for Query {
    fn from(text: String) -> Query {
        Query::new(&[&text])
    }
}
