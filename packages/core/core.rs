pub mod core {
    use std::{collections::HashMap, error::Error};

    use reqwest;
    use scraper::{Html, Selector};

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen::prelude::*;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_futures::JsFuture;

    pub async fn get_ogp(url: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let text: String = reqwest::get(url).await?.text().await?;
        let document = Html::parse_document(&text);
        let meta_selectors: Vec<Selector> = vec![
            String::from("title"),
            String::from("type"),
            String::from("image"),
            String::from("url"),
        ]
        .iter()
        .map(|e| {
            let s = format!("{}{}{}", "meta[property=\"og:", e, "\"]");
            Selector::parse(&s).unwrap()
        })
        .collect();

        let res =
            meta_selectors
                .iter()
                .fold(HashMap::<String, String>::new(), |mut acc, selector| {
                    if let Some(res) = document.select(&selector).next() {
                        acc.insert(
                            res.value().attr("property").unwrap().to_string(),
                            res.value().attr("content").unwrap().to_string(),
                        );
                    }

                    acc
                });

        Ok(res)
    }

    pub async fn get_og_title(url: &str) -> Result<String, Box<dyn Error>> {
        Ok(get_ogp(url).await?.get("og:title").unwrap().into())
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub async fn get_og_title_wasm(url: &str) -> Result<String, JsValue> {
        let promise = js_sys::Promise::resolve(&(get_og_title(url).await.unwrap()).into());
        let result = JsFuture::from(promise).await?.as_string().unwrap();

        Ok(result)
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
