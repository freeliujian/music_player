
pub struct StaticForStyles {
    pub header_height: String,
    pub player_bar_height: String,
}

pub fn styles_static () -> StaticForStyles{
    let header_height = String::from("60px");
    let player_bar_height = String::from("70px");
    StaticForStyles {
        header_height,
        player_bar_height
    }
}