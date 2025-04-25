use stylist::{Style, css};
use crate::config_provide::context::Theme;

pub fn styles(theme: &Theme) -> Style {
    let styles = css!(
        height: auto;
        border-top: 1px solid #d9d9d9;
        background-color: ${theme.color_bg_base};
        display: flex;
         .music_player_wrapper_left > .music-player-play-cover > .is_show {
            display: block;
        }
        .shadow {
            width: 100%;
            height: 100%;
            background: rgba(0,0,0,0.5);
            border-radius: 5px;
            display: none;
            animation: all 0.5s;
        }
        .music_player_wrapper_left{
            width: 200px;
            height: 100%;
            padding: 8px 12px;
            box-sizing: border-box;
            display: flex;
            gap: 14px;
            align-items: center;
        }
        .music-player-play-content{
            font-size: 14px;
            cursor: pointer;
        }
        .music-player-play-cover {
            width: 50px;
            height: 50px;
            border-radius: 5px;
            background: red;
        }
        .music_player_btn {
            width: 45px;
            height: 45px;
            background-color: rgba(255, 255, 255, 0.5);
            border-radius: 50%;
            display:flex;
            justify-content: center;
            align-items: center;
            cursor: pointer;
        }
        .music_player_btn:hover svg path{
            fill: #1677ff;
        }
        .music_player_btn :hover {
            color: #1677ff;
        }
        .music-player-play-name {
            margin-bottom: 10px;
        }
        .music-player-play-wrapper {
            flex: 1;
        }
        .music-player-play-action{
            display: flex;
            justify-content: center;
        }
        .music-player-other {
            width: 240px;
        }
        .music-player-play-main{
            flex: 1  auto;
        }
        .start-time{
            
        }
        .end-time{
            
        }
        .progress-wrapper {
            
        }
        .pause_play_content{
            width: 30px;
            height: 30px;
            display: flex;
            justify-content: center;
            align-items: center;
            border-radius: 50%;
            background: rgba(0,0,0,0.1);
        }
        .pause_play_content:hover {
            background: rgba(0,0,0,0.2);
        }
    );

    Style::new(styles).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}
