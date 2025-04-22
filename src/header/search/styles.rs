use stylist::{Style,css};

pub fn styles() -> Style {
  let styles = css!(
      display:inline-flex;
      justify-content: center;
      align-items: center;
      box-sizing: border-box;
      width: 100%;
      height: 24px;
  );
  
  Style::new(styles).unwrap_or_else(|e| {
      eprintln!("styled error is : {}", e);
      Style::new(css!("")).unwrap()
  })
}
