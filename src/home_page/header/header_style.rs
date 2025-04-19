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
            box-sizing: border-box;

        .header-left {
          width: 120px;
          display: flex;
          align-items: center;
          gap: 15px;
        }
        .nav-btn {
          width: 26px;
          height: 26px;
          cursor: pointer;
            display: flex;
            justify-content: center;
            align-items: center;
            background-color: rgba(0, 0, 0, 0.3);
            border-radius: 50%;
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
             gap: 5px;
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

        .user-avatar {
          width: 30px;
          height: 30px;
          border-radius: 50%;
            background-color: #bfbfbf;
            display: flex;
            justify-content: center;
            align-items: center;
            cursor: pointer;
        }
        .user-status {
            font-size: 12px;
            font-family: arial, sans-serif;
        }
    );

    Style::new(css).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}