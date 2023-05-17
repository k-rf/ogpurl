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

    pub async fn get_og_title(url: &str) -> Result<String, Box<dyn Error>> {
        Ok(get_ogp(url).await?.get("og:title").unwrap().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_get_ogp {
        use super::*;
        use std::collections::HashMap;

        #[tokio::test]
        async fn get_ogp_based_on_specified_url() {
            // Arrange
            let url: &str = "https://www.rust-lang.org/ja/learn/get-started";

            // Act
            let Ok(result) = core::get_ogp(&url).await else { panic!() };

            // Assert
            assert_eq!(result.get("og:title"), Some(&String::from("はじめに")));
        }

        #[tokio::test]
        async fn get_empty_when_ogp_is_not_exist() {
            // Arrange
            let url: &str = "https://example.com";

            // Act
            let Ok(result) = core::get_ogp(&url).await else { panic!() };

            // Assert
            assert_eq!(result, HashMap::new());
        }

        #[tokio::test]
        async fn error_when_url_is_invalid() {
            // Arrange
            let url: &str = "example.com";

            // Act
            let result = core::get_ogp(&url).await;

            // Assert
            assert!(result.is_err());
        }
    }

    mod test_get_og_title {
        use super::*;
        #[tokio::test]
        async fn get_og_title_based_on_specified_url() {
            // Arrange
            let url: &str = "https://www.rust-lang.org/ja/learn/get-started";

            // Act
            let Ok(result) = core::get_og_title(&url).await else { panic!() };

            // Assert
            assert_eq!(result, String::from("はじめに"));
        }
    }
}
