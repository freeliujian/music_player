use stylist::{Style,css};

pub fn styles() -> Style {
  let styles = css!(
    width: 140px;
    height: 204px;
    position: relative;
    cursor: pointer;
    list-style: none;
    :hover .cover  {
      display: block;
    }
    img {
      width: 100%;
    }
    .cover {
      width: 100%;
      height: 100%;
      background: rgba(0,0,0,0.5);
      position: absolute;
      left: 0;
      top: 0;
      display:none;
      animation: all 0.5s;
    }
    
  );
  
  Style::new(styles).unwrap_or_else(|e| {
      eprintln!("styled error is : {}", e);
      Style::new(css!("")).unwrap()
  })
}
