use serde_derive::{Deserialize, Serialize};


/**
 * {
  "sensor": "DHT22-01",
  "seq": 27,
  "temperature": 23.70000076,
  "humidity": 0.956999958
}
 */
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Temperature {
    pub sensor: String,
    #[serde(rename = "seq")]
    pub sequence: i32,
    pub temperature: f32,
    pub humidity: f32, 
}

