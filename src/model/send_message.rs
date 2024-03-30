use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageAuthor {
    pub name: String,
    pub username: String,
    pub email: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SendMessage {
    pub author: Option<MessageAuthor>,

    pub subject: String,

    pub message: String,
}
