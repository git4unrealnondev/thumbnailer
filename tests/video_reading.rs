extern crate core;

use file_format::FileFormat;
use std::io::Cursor;
use std::str::FromStr;
use thumbnailer::error::ThumbError;
use thumbnailer::{create_thumbnails, ThumbnailSize};

const VIDEO_BYTES: &[u8] = include_bytes!("assets/test.mp4");

#[test]
fn it_creates_thumbnails_for_mp4() {
    let reader = Cursor::new(VIDEO_BYTES);
    let result = create_thumbnails(
        reader,
        FileFormat::Mpeg4Part14Video,
        [
            ThumbnailSize::Small,
            ThumbnailSize::Medium,
            ThumbnailSize::Large,
        ],
    );

    match result {
        Ok(_) => {
            assert!(true);
        }
        Err(e) => match e {
            ThumbError::Unsupported(_) => {
                assert!(true, "ffmpeg is not installed");
            }
            e => {
                panic!("failed to create thumbnails {e}");
            }
        },
    }
}
