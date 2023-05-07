pub mod core {
    use std::{collections::HashMap, error::Error};

    use oglens::ogp::Ogp;
    use reqwest;

    pub async fn get_ogp(url: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let text: String = reqwest::get(url).await?.text().await?;

        // NOTE: デフォルトで `og:*` 系を取得するようになっているため、 `prefix_list` は空でよい
        let ogp: Ogp = Ogp::from(text.as_str(), vec![])?;

        Ok(ogp.items.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[tokio::test]
    async fn get_ogp_based_on_specified_url() {
        // Arrange
        let url: &str = "https://www.rust-lang.org/ja/learn/get-started";

        // Action & Assert
        if let Ok(result) = core::get_ogp(&url).await {
            assert_eq!(result.get("og:title"), Some(&String::from("はじめに")));
        }
    }

    #[tokio::test]
    async fn get_empty_when_ogp_is_not_exist() {
        // Arrange
        let url: &str = "https://example.com";

        // Action & Assert
        if let Ok(result) = core::get_ogp(&url).await {
            assert_eq!(result, HashMap::new());
        }
    }

    #[tokio::test]
    async fn error_when_url_is_invalid() {
        // Arrange
        let url: &str = "example.com";

        // Action
        let result = core::get_ogp(&url).await;

        // Assert
        assert!(result.is_err());
    }
}
