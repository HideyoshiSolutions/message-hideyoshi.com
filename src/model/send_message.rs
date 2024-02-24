use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use chrono::naive::serde::ts_seconds::deserialize as from_ts;



#[derive(Deserialize, Serialize)]
pub struct MessageAuthor {
    name: String,
    username: String,
    email: String
}


#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct SendMessage {
    author: Option<MessageAuthor>,

    subject: String,

    message: String,

    #[serde(deserialize_with = "from_ts")]
    timestamp: NaiveDateTime
}