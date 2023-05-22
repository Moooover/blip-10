use serde::{Deserialize, Deserializer, Serialize};
use serde_json;

use crate::Error;

use base64::{engine::general_purpose, Engine as _};

pub mod builder;

pub fn from_b64(b64: &str) -> Result<Boostagram, Error> {
    let json_raw = general_purpose::STANDARD.decode(b64)?;
    serde_json::from_slice(&json_raw).map_err(Error::from)
}

pub fn from_json(json: &str) -> Result<Boostagram, Error> {
    serde_json::from_str(json).map_err(Error::from)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Action {
    #[serde(rename = "stream")]
    Stream,
    #[serde(rename = "boost")]
    Boost,
    #[serde(rename = "lsat")]
    Lsat,
    #[serde(rename = "auto")]
    Auto,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Boostagram {
    pub podcast: Option<String>,

    #[serde(rename = "feedID")]
    pub feed_id: Option<usize>,

    pub url: Option<String>,

    pub guid: Option<String>,

    pub episode: Option<String>,

    #[serde(rename = "itemID")]
    #[serde(deserialize_with = "deserialize_item_id")]
    #[serde(default)]
    pub item_id: Option<usize>,

    pub episode_guid: Option<String>,

    pub time: Option<String>,

    pub ts: Option<usize>,

    pub action: Option<Action>,

    pub app_name: Option<String>,

    pub app_version: Option<String>,

    pub boost_link: Option<String>,

    pub message: Option<String>,

    pub name: Option<String>,

    pub sender_name: Option<String>,

    pub sender_id: Option<String>,

    pub signature: Option<String>,

    pub speed: Option<String>,

    pub uuid: Option<String>,

    //can't be larger for the lightning network
    pub value_msat_total: Option<u64>,

    pub reply_address: Option<String>,

    //TODO why string?
    pub reply_custom_key: Option<usize>,

    pub reply_custom_value: Option<String>,
}

fn deserialize_item_id<'de, D>(d: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).or(Ok(Option::None))
}

impl Boostagram {
    pub fn to_b64(&self) -> Result<String, Error> {
        let json = serde_json::to_vec(self)?;
        Ok(general_purpose::STANDARD.encode(json))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_fountain_invoice() {
        let boost_raw = "eyJhcHBfbmFtZSI6IkZvdW50YWluIiwidmFsdWVfbXNhdF90b3RhbCI6MTAwMDAwLCJuYW1lIjoiQWxieSBUZXN0IFVzZXIgUFVUIiwicG9kY2FzdCI6IlRlc3QgUG9kY2FzdCBBbmNob3IiLCJmZWVkSUQiOjYwMTU2NzEsImFjdGlvbiI6ImJvb3N0Iiwic2VuZGVyX2lkIjoiblNpcTdpZDc4SkFkSDl1WTFwSXkiLCJzZW5kZXJfbmFtZSI6IkBhbHdpbl9jb25zaGF4IiwibWVzc2FnZSI6InRlc3QiLCJpdGVtSUQiOiIxNDkzNDE1NDMwOSIsImJvb3N0X2xpbmsiOiJodHRwczovL2ZvdW50YWluLmZtL2VwaXNvZGUvMTQ5MzQxNTQzMDkiLCJlcGlzb2RlIjoidGhpcyBpcyBhIHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgbG9uZyBlcGlzb2RlIG5hbWUhISEiLCJ0cyI6MzI5LCJ0aW1lIjoiMDA6MDU6MjkifQ==";

        let result = from_b64(boost_raw);

        dbg!(&result);

        assert!(result.is_ok());

        let boostagram = result.unwrap();

        assert_eq!(boostagram.item_id, None);
    }
}
