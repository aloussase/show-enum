use core::panic;

use arraystring::{typenum::U20, ArrayString};

#[derive(Debug)]
pub struct Options {
    pub filename: ArrayString<U20>,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            filename: Default::default(),
            start: Default::default(),
            end: Default::default(),
        }
    }
}

impl Options {
    pub fn parse<T: Iterator<Item = String>>(cmdline: &mut T) -> Self {
        let mut options = Options::default();
        let mut filename_set = false;

        while let Some(ref arg) = cmdline.next() {
            match arg.as_ref() {
                "--start" | "-s" => {
                    if let Some(ref start) = cmdline.next() {
                        options.start = start
                            .parse::<u32>()
                            .ok()
                            .expect("expected start to be a number")
                            .into();
                    }
                }
                "--end" | "-e" => {
                    if let Some(ref end) = cmdline.next() {
                        options.end = end
                            .parse::<u32>()
                            .ok()
                            .expect("expected end to be a number")
                            .into();
                    }
                }
                filename if !filename_set => {
                    filename_set = true;
                    options.filename =
                        ArrayString::try_from_str(&filename).expect("failed to parse filename");
                }
                _ => {
                    panic!("usage: show-enum [--start] [--end] <filename>");
                }
            }
        }

        if options.filename.is_empty() {
            panic!("usage: show-enum [--start] [--end] <filename>");
        }

        options
    }
}
