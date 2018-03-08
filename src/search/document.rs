use super::Query;
use super::SEPARATORS;

#[derive(Debug)]
pub struct Document {
    tokens: Vec<String>,
}

impl Document {
    pub fn new(words: &[&str]) -> Document {
        let mut document = Document::default();
        document.extend(words);

        document
    }

    pub fn extend(&mut self, words: &[&str]) {
        for word in words {
            let word = word.to_lowercase();

            self.tokens.extend(
                word.split(|c| SEPARATORS.contains(c))
                    .filter(|token| !token.is_empty())
                    .map(|token| token.into()),
            );
        }
    }

    pub fn weight_for(&self, query: &Query) -> Option<usize> {
        let mut weight = 0;

        for query_token in query.tokens() {
            for document_token in &self.tokens {
                if document_token.contains(query_token) {
                    weight += query_token.len() * document_token.len();
                }
            }
        }

        match weight {
            0 => None,
            n => Some(n),
        }
    }
}

impl Default for Document {
    fn default() -> Document {
        Document {
            tokens: Vec::default(),
        }
    }
}
