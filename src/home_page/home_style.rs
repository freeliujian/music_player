use stylist::{Style, css};
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
          margin-top: ${static_for_styles.header_height};
           height: calc(100% ${"\u{0020}"} - ${"\u{0020}"} ${ static_for_styles.header_height}${"\u{0020}"} - ${"\u{0020}"}  ${static_for_styles.player_bar_height });
        }
        .main-content {
          flex: 1;
          margin-top: ${static_for_styles.header_height};
          overflow:hidden;
          width: 100%;
          height: calc(100% ${"\u{0020}"} - ${"\u{0020}"} ${ static_for_styles.header_height}${"\u{0020}"} - ${"\u{0020}"}  ${static_for_styles.player_bar_height });
        }
        .sidebar-right {
          width: 300px;
          height: 100%;
        }
        .player-bar {
          height: ${static_for_styles.player_bar_height};
          width: 100%;
          position: fixed;
          bottom: 0;
            background-color: #fff;
        }
    );


    Style::new(css).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}