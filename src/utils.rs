use serde::{Deserialize, Serialize};

pub fn get_data(username: String) -> Root {
    let query = "
    query ($username: String){
        MediaListCollection(userName: $username type: ANIME){
          lists {
            name
            status
            entries {
              id
              startedAt {
                year
                month
                day
              }
              completedAt{
                year
                month
                day
              }
              
              createdAt
              progress
              media{
                title {
                  romaji
                  english
                  native
                  userPreferred
                }
              }
            }
          }
        }
      }
    "
    .to_string();

    let some_more_data = serde_json::json!({
      "query": query,
      "variables": {
        "username": username
      }
    });
    let body = some_more_data;

    let response = minreq::post("https://graphql.anilist.co")
        .with_header("content-type", "application/json")
        .with_json(&body)
        .unwrap()
        .send()
        .unwrap();
    let data = response.json::<Root>();
    match data {
        Ok(data) => data,
        Err(err) => panic!("{}", err.to_string()),
    }
    // data.unwrap()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "MediaListCollection")]
    pub media_list_collection: MediaListCollection,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaListCollection {
    pub lists: Vec<List>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub name: String,
    pub status: String,
    pub entries: Vec<Entry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub id: i64,
    pub started_at: StartedAt,
    pub completed_at: CompletedAt,
    pub created_at: i64,
    pub progress: i64,
    pub media: Media,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartedAt {
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub day: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletedAt {
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub day: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub title: Title,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub romaji: String,
    pub english: Option<String>,
    pub native: String,
    pub user_preferred: String,
}
