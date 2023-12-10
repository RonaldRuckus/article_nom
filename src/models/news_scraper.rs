use fantoccini::{ClientBuilder, Locator};
use futures::{stream, StreamExt};

use crate::errors::gather_error::GatherError;

pub struct NewsScraper {
    client: Option<fantoccini::Client>,
}

impl NewsScraper {
    pub async fn new() -> Result<Self, fantoccini::error::NewSessionError> {
        let client = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await?;

        Ok(Self {
            client: Some(client),
        })
    }

    pub async fn close(&mut self) -> Result<(), fantoccini::error::CmdError> {
        if let Some(client) = self.client.take() {
            client.close().await
        } else {
            Ok(())
        }
    }

    /// # Purpose
    /// This function is used for gathering article data.
    /// Using Selenium it gathers the HTML content of all articles, or body elements if no articles are found.
    ///
    /// # Parameters
    /// * `url` - The URL of the article to be parsed
    /// * `clean_links` - Whether or not to remove links from the article
    ///
    /// # Returns
    /// * `String` - The article text in Markdown
    ///
    /// # Notes
    /// Geckodriver must be running on port 4444 for this function to work.
    pub async fn get_elements(&self, url: &str) -> Result<Vec<String>, GatherError> {
        let client = match self.client.as_ref() {
            Some(client) => client,
            None => return Err(GatherError::SessionDropped()),
        };

        client.goto(url).await?;

        let current_url = client.current_url().await?;
        if current_url.as_ref() != url {
            eprintln!("Warning: URL redirect from {} to {}", url, current_url);
        }

        let elements = match client.find_all(Locator::Css("article")).await {
            Ok(articles) if !articles.is_empty() => articles,
            _ => {
                println!("Article not found, using body");
                client.find_all(Locator::Css("body")).await?
            }
        };

        // Eagerly extract the HTML from the elements
        let html_contents = stream::iter(elements)
            .then(|e| async move {
                let html = e.html(true).await;
                html.unwrap_or_else(|_| String::new())
            })
            .collect::<Vec<_>>()
            .await;

        Ok(html_contents)
    }
}
