use std::{fmt::Display, path::Path};

use audiotags::Tag;
use base64::{Engine, prelude::BASE64_STANDARD};

pub struct SongData {
    pub title: String,
    pub album_title: String,
    pub artist: String,
    pub length: Option<f64>,
    pub cover: Option<Box<[u8]>>,
}

pub fn get_meta(path: impl AsRef<Path>) -> SongData {
    println!("{:?}", path.as_ref());
    let tag = Tag::new().read_from_path(path.as_ref()).unwrap();

    let cover = tag
        .album_cover()
        .map(|c| c.data.to_vec().into_boxed_slice())
        .or_else(|| {
            metaflac::Tag::read_from_path(path.as_ref())
                .ok()?
                .pictures()
                .next()
                .map(|c| c.data.clone().into_boxed_slice())
        });

    SongData {
        title: tag.title().unwrap().to_owned(),
        album_title: tag.album_title().unwrap().to_owned(),
        artist: tag.artist().unwrap().to_owned(),
        length: tag.duration(),
        cover,
    }
}

impl Display for SongData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(image) = self.cover.clone() {
            writeln!(
                f,
                "\x1b]1337;File=;inline=1;height=256px;width=256px:{}\x07",
                BASE64_STANDARD.encode(image)
            )?;
        }

        write!(
            f,
            "{title} from {album} by {artist}",
            title = self.title,
            artist = self.artist,
            album = self.album_title,
        )?;

        if let Some(dur) = self.length {
            write!(
                f,
                " ({minutes}m{seconds}s)",
                minutes = (dur / 60.0).floor(),
                seconds = (dur % 60.0).floor()
            )?;
        }

        Ok(())
    }
}
