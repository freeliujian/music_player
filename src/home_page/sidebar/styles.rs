use stylist::{Style, css};
use crate::config_provide::context::Theme;
use crate::styles::{styles_static};

pub fn styles(theme: &Theme, is_clicked: &String) -> Style {
    let static_for_styles = styles_static();
    let font_weight_important ="600!important";
    let css = css!(
          width: 200px;
          margin-top: ${static_for_styles.header_height};
           height: calc(100% ${"\u{0020}"} - ${"\u{0020}"} ${ static_for_styles.header_height}${"\u{0020}"} - ${"\u{0020}"}  ${static_for_styles.player_bar_height });
        background-color: ${theme.color_bg_base};
        color: ${theme.color_text_base};
        border-right: solid 1px #d9d9d9;
        padding: 12px 8px;
        box-sizing: border-box;
        .menu {

        }
        .menu > .active {
            font-size: 18px;
            font-weight: ${font_weight_important};
        }
        .menu>.menu-list {
            box-sizing: border-box;
            height: 42px;
            padding: 12px 8px;
            cursor: pointer;
            color: #595959;
            font-weight: ${if *is_clicked==String::from("发现音乐") { "500" } else { "normal" }};
        }
        .menu-list :hover {
            background-color: rgba(0,0,0,0.1);
            color: ${theme.color_text_base};
        }
        
    );


    Style::new(css).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}