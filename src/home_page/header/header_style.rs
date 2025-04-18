use stylist::{Style,css};
use crate::styles::{styles_static};

pub fn styles() -> Style {
    let static_for_styles = styles_static();

    let css = css!(
        height: ${static_for_styles.header_height};
          width: 100%;
          display: flex;
          align-items: center;
          padding: 0 20px;
          background-color: rgba(0, 0, 0, 0.3);
          backdrop-filter: blur(10px);
          position: fixed;
          top: 0;
          z-index: 1000;

        .header-left {
          width: 240px;
          display: flex;
          align-items: center;
          gap: 15px;
        }
        .nav-btn {
          width: 20px;
          height: 20px;
          cursor: pointer;
        }
        .log_img {
          height: 30px;
            display: flex;
            justify-content: center;
            align-items: center;
            cursor: pointer;
        }
        .log_img > img {
            height: 100%;
            margin-right: 10px;
        }
        .log_img > span {
            color: white;
        }
        .header-center {
          flex: 1;
          display: flex;
          justify-content: center;
        }

        .search-box {
          width: 160px;
          height: 32px;
          border-radius: 16px;
          padding: 0 15px;
          font-size: 14px;
          transition: width 0.3s;
        }

        .search-box:focus {
          width: 300px;
        }

        .header-right {
          width: 240px;
          display: flex;
          justify-content: flex-end;
          align-items: center;
          gap: 20px;
        }

        .user-avatar {
          width: 30px;
          height: 30px;
          border-radius: 50%;
        }
    );

    Style::new(css).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}