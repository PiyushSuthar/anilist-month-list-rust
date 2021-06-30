use crate::models::Root;

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
