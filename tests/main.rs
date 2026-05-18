use serde_json::Value;
use std::error::Error;
use std::time::Duration;
use systemlab21::routes::check_conflicts::Output;

#[tokio::test]
async fn main() -> Result<(), Box<dyn Error>> {
    let body_raw = r#"
    {
    "station_graph": [
     {"start": "Station West", "end": "Entry Signal West" },
     {"start": "Entry Signal West", "end": "Point 1" },
     {"start": "Point 1", "end": "Exit Signal West 1" },
     {"start": "Point 1", "end": "Exit Signal West 2" },
     {"start": "Exit Signal West 1", "end": "Exit Signal East 1" },
     {"start": "Exit Signal West 2", "end": "Exit Signal East 2" },
     {"start": "Exit Signal East 1", "end": "Point 2" },
     {"start": "Exit Signal East 2", "end": "Point 2" },
     {"start": "Point 2", "end": "Entry Signal East" },
     {"start": "Entry Signal East", "end": "Station East" }
    ],
     "routes": [
     {"start": "Entry Signal West", "end": "Exit Signal East 1",
    "occupied": false },
     {"start": "Entry Signal West", "end": "Exit Signal East 2",
    "occupied": false },
     {"start": "Exit Signal East 1", "end": "Station East", "occupied":
    false },
     {"start": "Exit Signal East 2", "end": "Station East", "occupied":
    false },
     {"start": "Entry Signal East", "end": "Exit Signal West 1",
    "occupied": false },
     {"start": "Entry Signal East", "end": "Exit Signal West 2",
    "occupied": false },
     {"start": "Exit Signal West 1", "end": "Station West", "occupied":
    true },
     {"start": "Exit Signal West 2", "end": "Station West", "occupied":
    false }
     ],
     "check_route": {"start": "Entry Signal West", "end": "Exit Signal East 2"}
    }
    "#;

    let body: Value = serde_json::from_str(body_raw).unwrap();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(20))
        .build()?;

    let url = "http://localhost:3000/check_conflicts";

    let resp: Output = client.post(url).json(&body).send().await?.json().await?;
    assert!(!resp.success);

    Ok(())
}
