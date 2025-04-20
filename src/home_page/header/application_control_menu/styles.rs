use stylist::{Style,css};

pub fn styles() -> Style {
    let css = css!(
        flex: 1;
        display: flex;
        gap: 12px;
    );

    Style::new(css).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}