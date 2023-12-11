#[derive(Debug)]
pub struct NewsArticle {
    pub url: String,
    pub headline: String,
}

impl NewsArticle {
    // Converts to just the headlines
    pub fn to_string(&self) -> String {
        self.headline.clone()
    }
}