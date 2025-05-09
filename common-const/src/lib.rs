pub mod window_control_common_const {
    use serde::{Serialize, Deserialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum ControlWindowType {
        Close,
        Hide,
        Maximize,
        Normal,
    }
}
