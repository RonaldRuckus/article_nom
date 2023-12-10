use errors::gather_error::GatherError;
use futures::{stream, StreamExt};
use html2md::parse_html;

use models::{
    html_cleaner::{CleanerConfig, HtmlCleaner},
    news_article::NewsArticle,
    news_scraper::NewsScraper,
};

use regex::Regex;

mod models {
    pub mod html_cleaner;
    pub mod news_article;
    pub mod news_scraper;
}

mod errors {
    pub mod gather_error;
}

/// # Purpose
/// Parses and extracts the URL and headline from a string into a NewsArticle
///
/// # Parameters
/// * `text` - The markdown text to be parsed
///
/// # Returns
/// * `Option<NewsArticle>` - The parsed NewsArticle
async fn extract_url_headline(text: &str) -> Option<NewsArticle> {
    let url_regex = Regex::new(r"\[.*?\]\((.*?)\)").unwrap();
    let headline_regex = Regex::new(r"#### (.*?) ####").unwrap();

    let url_prefix = "https://news.google.com";

    let first_url = url_regex
        .captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string());

    let headline = headline_regex
        .captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string());

    match (first_url, headline) {
        (Some(url), Some(headline)) => {
            let full_url = format!("{}{}", url_prefix, &url[1..]);
            Some(NewsArticle {
                url: full_url,
                headline,
            })
        }
        _ => None,
    }
}

/// # Purpose
/// Reduces a vector of strings (articles) into a single String
///
/// # Parameters
/// * `articles` - The articles to be folded
///
/// # Returns
/// * `String` - The folded articles
async fn fold_articles(articles: &Vec<String>) -> String {
    stream::iter(articles)
        .fold(String::new(), |mut acc, text| async move {
            if !acc.is_empty() {
                acc.push_str("\n\nNEW ARTICLE: ");
            }
            acc.push_str(text);
            acc
        })
        .await
}

/// # Purpose
/// Cleans the HTML and potentially converts it to Markdown
///
/// # Parameters
/// * `elements` - The elements as Strings to be cleaned
/// * `html_cleaner_config` - The configuration for the HTML cleaner
/// * `to_markdown` - Whether or not to convert the HTML to Markdown
///
/// # Returns
/// * `Vec<String>` - A vector of the cleaned HTML
async fn clean_html(
    elements: &Vec<String>,
    html_cleaner_config: &CleanerConfig,
    to_markdown: bool,
) -> Vec<String> {
    println!("CLEAN HTML ELEMENTS: {}", elements.len());

    stream::iter(elements)
        .filter_map(|html| {
            let local_config = html_cleaner_config.clone();
            async move {
                let html_cleaner = HtmlCleaner::new().apply_config(&local_config);
                let cleaned = html_cleaner.clean(html);

                Some(if to_markdown {
                    parse_html(&cleaned)
                } else {
                    cleaned
                })
            }
        })
        .collect()
        .await
}

/// # Purpose
/// Opens the URL and returns the article's elements
///
/// # Parameters
/// * `url` - The URL of the article to be parsed
///
/// # Returns
/// * `Result<Vec<Element>, GatherError>` - The article's elements
async fn gather_article_elements(url: &str) -> Result<Vec<String>, GatherError> {
    let mut scraper = NewsScraper::new().await?;
    let elements = scraper.get_elements(&url).await?;
    let _ = scraper.close().await;
    Ok(elements)
}

/// # Purpose
/// Grabs an article from a URL
///
/// # Parameters
/// * `url` - The URL of the article to be parsed
/// * `html_cleaner_config` - The configuration for the HTML cleaner
///
/// # Returns
/// * `Result<String, GatherError>` - The article text in Markdown
pub async fn gather_article(
    url: &str,
    html_cleaner_config: &CleanerConfig,
) -> Result<String, GatherError> {
    let elements = gather_article_elements(&url).await?;
    let parsed_text = clean_html(&elements, &html_cleaner_config, true).await;
    let folded_text = fold_articles(&parsed_text).await;

    Ok(folded_text)
}

/// # Purpose
/// Grabs articles found on Google News
/// Cleans them, converts to markdown, and filters them into a vector of NewsArticles
///
/// # Parameters
/// * `search_query` - The search query to be used on Google News
///
/// # Returns
/// * `Result<Vec<NewsArticle>, GatherError>` - The articles found
pub async fn gather_google_articles(search_query: &str) -> Result<Vec<NewsArticle>, GatherError> {
    let url = format!(
        "https://news.google.com/search?q={}&hl=en-US&gl=US&ceid=US%3Aen",
        search_query
    );
    let elements = gather_article_elements(&url).await?;

    println!("Found elements: {:?}", elements.len());

    let config = CleanerConfig {
        remove_script_tags: true,
        remove_a_tags: false,
        remove_img_tags: true,
        remove_source_tags: false
    };

    let parsed_text = clean_html(&elements, &config, true).await;

    println!("Parsed text: {:?}", parsed_text);

    let mut articles = Vec::new();
    for text in parsed_text {
        if let Some(article) = extract_url_headline(&text).await {
            articles.push(article);
        }
    }

    Ok(articles)
}

#[cfg(test)]
mod tests {

    use super::*;

    /// # Purpose
    /// Grabs articles found on Google News
    ///
    /// # Expects
    /// A vector of unstructured articles as Strings
    #[tokio::test]
    async fn test_get_google_news() {
        let news_articles = gather_google_articles("AI").await.unwrap();

        println!("{:?}", news_articles);

        assert!(news_articles.len() > 0);
    }

    /// # Purpose
    /// Parses a single example vector of Google News articles
    ///
    /// # Expects
    /// A NewsArticle with a URL and headline
    #[tokio::test]
    async fn parse_google_news_article() {
        let example = "[](./articles/CBMiRWh0dHBzOi8vd3d3LndpcmVkLmNvbS9zdG9yeS9ob3ctdG8tdXNlLWdvb2dsZS1nZW1pbmktYWktYmFyZC1jaGF0Ym90L9IBAA?hl=en-US&gl=US&ceid=US%3Aen)\n\nWIRED\n\nMore\n\n[\n\n#### How to Use Google's Gemini AI Right Now in Its Bard Chatbot ####\n\n](./articles/CBMiRWh0dHBzOi8vd3d3LndpcmVkLmNvbS9zdG9yeS9ob3ctdG8tdXNlLWdvb2dsZS1nZW1pbmktYWktYmFyZC1jaGF0Ym90L9IBAA?hl=en-US&gl=US&ceid=US%3Aen)\n\n4 days ago\n---\n\nReece RogersBy Reece Rogers";

        let article = extract_url_headline(&example).await.unwrap();

        println!("Last vector: {:?}", article);

        assert!(article.headline.len() > 0 && article.url.len() > 0);
    }

    /// # Purpose
    /// Retrieves the article text from a URL
    /// 
    /// # Expects
    /// A string of text
    #[tokio::test]
    async fn test_gather_article() {
        let url = "https://news.google.com/articles/CBMiRWh0dHBzOi8vd3d3LndpcmVkLmNvbS9zdG9yeS9ob3ctdG8tdXNlLWdvb2dsZS1nZW1pbmktYWktYmFyZC1jaGF0Ym90L9IBAA?hl=en-US&gl=US&ceid=US%3Aen";

        let config = CleanerConfig {
            remove_script_tags: true,
            remove_a_tags: true,
            remove_img_tags: true,
            remove_source_tags: true
        };

        let article = gather_article(&url, &config).await.unwrap();

        println!("Article: {}", article);

        assert!(article.len() > 0);
    }
}
