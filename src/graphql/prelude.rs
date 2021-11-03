use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use wasm_bindgen::JsValue;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/graphql/nearby_stations.graphql",
    response_derives = "Debug"
)]
pub struct NearbyStations;

fn log(s: &str) {
    web_sys::console::log_1(&JsValue::from_str(s));
}

pub async fn load_nearby_stations(
    latitude: f64,
    longitude: f64,
) -> Result<graphql_client::Response<nearby_stations::ResponseData>, JsValue> {
    let url = "https://sapi.tinykitten.me/graphql";
    let variables = nearby_stations::Variables {
        latitude,
        longitude,
    };

    let client = reqwest::Client::new();

    post_graphql::<NearbyStations, _>(&client, url, variables)
        .await
        .map_err(|err| {
            log(&format!(
                "Could not fetch nearby stations. error: {:?}",
                err
            ));
            JsValue::NULL
        })
}
