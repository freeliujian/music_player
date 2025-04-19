use stylist::{Style,css};

pub fn styles() -> Style {
    let css = css!(
          width: 360px;
          display: flex;
          align-items: center;
            justify-content: center;
            flex-direction: row;
          gap: 6px;
            color:white;
            cursor: pointer;
        .config_wrapper {
            display: flex;
            flex-direction: row;
            gap:15px;
        }
    );

    Style::new(css).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}