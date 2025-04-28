use stylist::{Style, css};
use crate::config_provide::context::Theme;

pub fn styles(theme: &Theme) -> Style {

    let styles = css!(
      r#"
          padding: 0px 2px;
          :nth-child(even) {
            background: #f5f5f5;
          }
          .music-list-content-wrapper {
              display: flex;
              font-size: 14px;
              cursor: pointer;
              color: #595959;
              padding: 8px 0px;              
          }
          .music-list-content-wrapper:hover {
            background: #f0f0f0;
          }
          .name {
              flex:1 auto;
          }
          .author {
              width: 80px;
          }
          .time {
              width: 90px;
          }
          .music-list-content-wrapper:hover .name {
              color: #141414;
          }
          .music-list-content-wrapper:hover .author {
              color: #141414;
          }
          .music-list-content-wrapper:hover .time {
              color: #141414;
          }
      "#
    );

    Style::new(styles).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}
