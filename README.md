## Article Nommer

Exposes two functions:

### gather_google_articles
```
gather_google_articles(
    search_query: &str
) -> Result<Vec<NewsArticle>, GatherError>
```

Uses Google News Trending to gather articles. Returns a vector of `NewsArticle` structs.
```
pub struct NewsArticle {
    pub url: String,
    pub headline: String,
}
```

### gather_article
```
gather_article(
    url: &str,
    html_cleaner_config: &CleanerConfig,
) -> Result<String, GatherError>
```

Visits the article page and returns the article text. Returns as Markdown.
A cleaner config is required to clean tags.

```
pub struct CleanerConfig {
    pub remove_script_tags: bool,
    pub remove_a_tags: bool,
    pub remove_img_tags: bool,
    pub remove_source_tags: bool
}
```

## Requirements
geckodriver must be running on port 4444.