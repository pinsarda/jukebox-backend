pub mod user;
pub mod music;

use paperclip::actix::Apiv2Schema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct Id {
    pub id: i32
}
