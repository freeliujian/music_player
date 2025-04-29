use stylist::{Style,css};

pub fn styles() -> Style {
  let styles = css!(
    padding:16px 24px;
    .title {
      font-size: 20px;
      font-weight: 500;
      margin-bottom: 16px;
      padding-top: 16px;
    }
    .carousel {
      width: 100%;
      height: 320px;
      background: red;
    }
    .song-list {
      display: flex;
    }
  );
  
  Style::new(styles).unwrap_or_else(|e| {
      eprintln!("styled error is : {}", e);
      Style::new(css!("")).unwrap()
  })
}
