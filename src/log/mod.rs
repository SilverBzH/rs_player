use std::sync::{Arc, Mutex};

use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::ListItem;

use super::lazy_static::lazy_static;

pub type LogType = Arc<Mutex<Vec<ListItem<'static>>>>;

lazy_static! {
    pub static ref LOGS: LogType = Arc::new(Mutex::new(Vec::new()));
}

macro_rules! build_log {
    ($color: expr, $tag: expr, $msg: expr) => {
        let style = Style::default().fg($color).bg(Color::Reset);
        let tag = format!("{}: ", $tag);
        let info_tag = Span::styled(tag, style);
        let info_message = Span::styled($msg, Style::default());
        let log_content = Spans::from(vec![info_tag, info_message]);
        if let Ok(mut guard) = LOGS.try_lock() {
            guard.push(ListItem::new(log_content));
        }
    };
}

pub struct Log {}

impl Log {
    pub fn info(msg: String) {
        build_log!(Color::Green, "INFO", msg);
    }

    pub fn debug(msg: String) {
        build_log!(Color::Blue, "DEBUG", msg);
    }

    pub fn warn(msg: String) {
        build_log!(Color::Yellow, "WARN", msg);
    }

    pub fn error(msg: String) {
        build_log!(Color::Red, "ERROR", msg);
    }
}
