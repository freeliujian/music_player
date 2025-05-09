use stylist::{Style,css};

pub fn styles() -> Style {
  let styles = css!(
    width: 140px;
    height: 204px;
    position: relative;
    cursor: pointer;
    list-style: none;
    .wrapper:hover div.cover {
      opacity: 1;
    }
    img {
      width: 100%;
    }
    .cover {
      width: 100%;
      height: 40px;
      padding: 8px 4px;
      color: #fff;
      font-size: 12px;
      background: rgba(0,0,0,0.5);
      position: absolute;
      left: 0;
      top: 0;
      opacity: 0;
      transition: all 0.5s;
      box-sizing: border-box;
    }
    .card_text {
      margin: 8px 0 3px;
      font-size: 14px;
    }
    
  );
  
  Style::new(styles).unwrap_or_else(|e| {
      eprintln!("styled error is : {}", e);
      Style::new(css!("")).unwrap()
  })
}
