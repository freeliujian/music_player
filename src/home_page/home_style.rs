use stylist::{Style,css};
use crate::styles::{styles_static};

pub fn styles() -> Style {
    let static_for_styles = styles_static();

    let css = css!(
        .music-header {
          height: ${static_for_styles.header_height};
          width: 100%;
          display: flex;
          align-items: center;
          padding: 0 20px;
          background-color: rgba(0, 0, 0, 0.3);
          backdrop-filter: blur(10px);
          position: fixed;
          top: 0;
          z-index: 1000;
        }
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