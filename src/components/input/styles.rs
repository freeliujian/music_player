use stylist::{Style, css};

pub fn styles() -> Style {
    let styles = css!(
        position: relative;
        display: inline-flex;
        width: 100%;
        min-width: 0;
        padding: 4px 11px;
        color: rgba(0, 0, 0, 0.85);
        font-size: 14px;
        background-color: rgba(0, 0, 0, 0.2);
        border-radius: 20px;
        transition: all 0.3s;
        cursor: text;
        :hover {
            border-color: #40a9ff;
        }
        :focus-within {
            border-color: #40a9ff;
            box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.2);
        }
        .input {
            width: 100%;
            min-width: 0;
            padding: 1px;
            color: #fff;
            font-size: 14px;
            background-color: transparent;
            border: none;
            outline: none;
            box-shadow: none;
        }
        .input:focus {
            outline: none;
        }
        .input-prefix, .input-suffix {
            display: flex;
            flex: none;
            align-items: center;
        }
        .input-prefix {
            margin-right: 4px;
        }
        .input-suffix {
            margin-left: 4px;
        }
        .input-disabled {
            background-color: #f5f5f5;
            cursor: not-allowed;
            opacity: 1;
        }
        .input-sm {
            padding: 0 7px;
            font-size: 12px;
        }
        .input-lg {
            padding: 6.5px 11px;
            font-size: 16px;
        }
    );
    
    Style::new(styles).unwrap_or_else(|e| {
        eprintln!("styled error is : {}", e);
        Style::new(css!("")).unwrap()
    })
}
