use stylist::{Style,css};

pub fn styles() -> Style {
  let styles = css!(
    padding: 0px 24px;
    overflow-y: scroll;
    height: 100%;
    .title {
      font-size: 20px;
      font-weight: 500;
      margin-bottom: 16px;
      padding-top: 16px;
    }
    .carousel {
      width: 100%;
      height: 320px;
      background: #31c27c;
    }
    .song-list {
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
    }
    .top_list_wrapper{
      width: 100%;
      display: flex;
      gap: 32px;
      flex-direction: row;
    }
  );

  Style::new(styles).unwrap_or_else(|e| {
      eprintln!("styled error is : {}", e);
      Style::new(css!("")).unwrap()
  })
}
