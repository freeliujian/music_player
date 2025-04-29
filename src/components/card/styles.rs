use stylist::{Style,css};

pub fn styles() -> Style {
  let styles = css!(
  
  );
  
  Style::new(styles).unwrap_or_else(|e| {
      eprintln!("styled error is : {}", e);
      Style::new(css!("")).unwrap()
  })
}
