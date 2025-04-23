use stylist::{Style, css};

pub fn styles() -> Style {
    let styles = css!(
        display: flex;
        align-items: center;
        width: 100%;
        gap: 10px;
        padding: 0 10px;
        
        .time-display {
            font-size: 12px;
            color: #999;
            min-width: 40px;
            text-align: center;
        }
        
        .progress-bar-wrapper {
            position: relative;
            flex-grow: 1;
            height: 4px;
            cursor: pointer;
        }
        
        .progress-bar-bg {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background-color: #e0e0e0;
            border-radius: 2px;
            overflow: hidden;
        }
        
        .progress-bar-progress {
            position: relative;
            height: 100%;
            background-color: #c70c0c;
            border-radius: 2px;
            transition: width 0.1s linear;
        }
        
        .progress-bar-thumb {
            position: absolute;
            right: 0;
            top: 50%;
            transform: translate(50%, -50%);
            width: 12px;
            height: 12px;
            border-radius: 50%;
            background-color: #c70c0c;
            opacity: 0;
            transition: opacity 0.2s;
        }
        
        .progress-bar-wrapper:hover .progress-bar-thumb,
        .progress-bar-wrapper:active .progress-bar-thumb {
            opacity: 1;
        }

    );

    Style::new(styles).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}
