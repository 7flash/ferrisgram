// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};

/// Describes a Web App.
/// <https://core.telegram.org/bots/api#webappinfo>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in Initializing Web Apps
    pub url: String,
}
