use stylist::{Style, css};
use crate::config_provide::context::Theme;

pub fn styles(theme: &Theme) -> Style {
    let styles = css!(
        height: auto;
        border-top: 1px solid #d9d9d9;
        background-color: ${theme.color_bg_base};
    );

    Style::new(styles).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}
