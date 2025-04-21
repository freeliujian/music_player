pub mod window_control_common_const {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum ControlWindowType {
        Close,
        Hide,
        Maximize,
        Normal,
    }
}
