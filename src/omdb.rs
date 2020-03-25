use crate::SearchData;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct OMDB {
    api_key: &'static str
}

impl OMDB {
   pub fn new(api_key: &'static str) -> OMDB {
        OMDB {
            api_key
        }
    }

    pub async fn search(&self, form_data: &SearchData) -> Result<String> {
        let url = format!("https://omdbapi.com/?apikey={}&s={}&type={}",
                          self.api_key,
                          form_data.search,
                          form_data.entertainment_type
        );
        let resp = reqwest::get(&url).await?.text().await?;

        Ok(resp)
    }
}
