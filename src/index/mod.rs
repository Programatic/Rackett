use async_trait::async_trait;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::Result as SJResult;
//TODO: use torrent_name_parser::Metadata;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub enum EntertainmentType {
    MOVIE,
    TV
}

#[async_trait]
pub trait Indexer {
    fn new() -> Self;
    async fn search(&self, entertainment_type: EntertainmentType, query: &str) -> Result<Vec<Torrent>>;
}

#[derive(Debug, Serialize)]
pub struct Torrent {
    name: String,
    seeders: usize,
    leechers: usize,
    magnet: String
    //TODO: metadata: Metadata
}

#[derive(Debug)]
pub struct PirateBay {
    host: &'static str,
    search_str: &'static str
}

#[async_trait]
impl Indexer for PirateBay {
   fn new() -> PirateBay {
        PirateBay {
            host: "https://www.pirate-bay.net",
            search_str: "https://tpb.party/s/?q="
        }
    }

    async fn search(&self, entertainment_type: EntertainmentType, query: &str) -> Result<Vec<Torrent>> {
        let mut torrents = Vec::new();
        let url = match entertainment_type {
            EntertainmentType::MOVIE => format!("{}{}", self.search_str, query),
            EntertainmentType::TV => format!("{}{}", self.search_str, query),
        };
        let res = reqwest::get(url.as_str()).await?.text().await?;
        let fragment = Html::parse_document(res.as_str());

        let selector = Selector::parse("#searchResult > tbody > tr").unwrap();
        let name_selector = Selector::parse(".detLink").unwrap();
        let seeders_selector = Selector::parse("td:nth-child(3)").unwrap();
        let leechers_selector = Selector::parse("td:nth-child(4)").unwrap();
        let magnet_selector = Selector::parse("td:nth-child(2) > a").unwrap();

        let links = fragment.select(&selector);
        for element in links {
            let name = element.select(&name_selector).next();
            let seeders = element.select(&seeders_selector).next();
            let leechers = element.select(&leechers_selector).next();
            let magnet = element.select(&magnet_selector).next();

            if let Some(magnet) = magnet {
                match (name, seeders, leechers, magnet.value().attr("href")) {
                    (Some(name), Some(seeders), Some(leechers), Some(magnet)) => {
                        let tor = Torrent {
                            name: name.inner_html(),
                            seeders: seeders.inner_html().parse::<usize>().unwrap(),
                            leechers: leechers.inner_html().parse::<usize>().unwrap(),
                            magnet: String::from(magnet)
                            //TODO: Implement: metadata: Metadata::from(name.inner_html().as_str()).unwrap()
                        };

                        torrents.push(tor);
                    }
                    _ => {}
                }
            }
        }

        Ok(torrents)
    }
}
