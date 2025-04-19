use stylist::{Style,css};

pub fn styles() -> Style {
    let css = css!(
        display:inline-flex;
        align-items: center;
        color:inherit;
        font-style:normal;
        line-height:0;
        text-align:center;
        text-transform:none;
        text-rendering:optimizeLegibility;
        -webkit-font-smoothing:antialiased;
        svg {
            display: inline-block;

        }

    );

    Style::new(css).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}