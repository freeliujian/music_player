use stylist::{Style,css};
use crate::styles::{styles_static};

pub fn styles() -> Style {
    let static_for_styles = styles_static();

    let css = css!(
        .app-container {
          width: 100vw;
          height: 100vh;
          display: flex;
        }
        .sidebar-left {
          width: 200px;
          margin-top: 50px;
          height: calc(100% - ${static_for_styles.header_height}px - ${static_for_styles.player_bar_height }px);
        }
        .main-content {
          flex: 1;
           margin-top: 50px;
          height: calc(100% - ${static_for_styles.header_height} - ${static_for_styles.player_bar_height }px);
        }

        .sidebar-right {
          width: 300px;
          height: 100%;
        }

        .player-bar {
          height: ${static_for_styles.player_bar_height }px;
          width: 100%;
          position: fixed;
          bottom: 0;
        }
    );

    Style::new(css).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}