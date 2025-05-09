use stylist::{Style,css};

pub fn styles() -> Style {
  let styles = css!(
    width: 164px;
    height: 567px;
    position: relative;
    cursor: pointer;
    list-style: none;
    color: white;
    text-align: center;
    padding: 18px 4px;
    box-sizing: border-box;
    .title { 
      font-size: 22px;
    }
  );
  
  Style::new(styles).unwrap_or_else(|e| {
      eprintln!("styled error is : {}", e);
      Style::new(css!("")).unwrap()
  })
}
