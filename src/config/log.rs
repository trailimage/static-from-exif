//! RON logs

use super::load_ron;
use crate::{
    models::{Blog, Photo, PhotoPath, Post, TagPhotos},
    tools::write_result,
};
use chrono::{DateTime, FixedOffset, Local};
use hashbrown::HashMap;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// File that stores photo tag information and last process time
static LOG_FILE: &str = "log.ron";

/// Log processed photo information per post folder to determine when
/// re-processing is necessary.
///
/// Re-rendering is triggered when
///
/// - the configuration file or a photo has a modified date newer than
///   the `as_of` time
/// - the number of photos has changed
/// - adjacent post paths have changed
///
/// Re-generating an image is triggered when
///
/// - its modified date is newer than the `as_of` time
///
#[derive(Serialize, Deserialize, Debug)]
pub struct PostLog {
    #[serde(default)]
    pub next_path: Option<String>,

    #[serde(default)]
    pub prev_path: Option<String>,

    /// Date of first relevant (not an outlier) photo in folder
    pub happened_on: Option<DateTime<FixedOffset>>,

    /// Timestamp when post data were last loaded
    pub as_of: i64,

    /// Number of photos in the post. If this changes then the post needs to be
    /// re-rendered.
    pub photo_count: usize,

    /// Even if post hasn't changed, its cover photo may be required to re-
    /// render category pages it's part of
    pub cover_photo: Option<Photo>,

    /// Photo tags keyed by their slug to the photos they were assigned to.
    /// These are logged so that photo tag pages can be regenerated.
    pub tags: HashMap<String, TagPhotos<u8>>,

    /// Whether post source files have changed since they were last read
    #[serde(skip)]
    pub files_have_changed: bool,
}

impl PostLog {
    /// Save information about loaded photos to avoid unecessary re-processing
    pub fn write(root: &Path, post: &Post) {
        let log = PostLog {
            prev_path: post.prev_path.clone(),
            next_path: post.next_path.clone(),
            happened_on: post.happened_on,
            photo_count: post.photo_count,
            as_of: Local::now().timestamp(),
            tags: post.tags.clone(),
            files_have_changed: false,
            cover_photo: post.cover_photo().map(|p| p.clone()),
        };
        let path = root.join(&post.path).join(LOG_FILE);
        let pretty = PrettyConfig {
            depth_limit: 4,
            ..PrettyConfig::default()
        };

        write_result(&path, || to_string_pretty(&log, pretty));
    }

    /// Load log file from path
    pub fn load(path: &Path) -> Option<Self> {
        load_ron(path, LOG_FILE, false)
    }

    pub fn empty() -> PostLog {
        PostLog {
            next_path: None,
            prev_path: None,
            happened_on: None,
            as_of: 0,
            photo_count: 0,
            tags: HashMap::new(),
            files_have_changed: true,
            cover_photo: None,
        }
    }

    /// Whether logged values differ from current post values
    pub fn differs(&self, post: &Post) -> bool {
        self.prev_path != post.prev_path || self.next_path != post.next_path
    }
}

impl Clone for PostLog {
    fn clone(&self) -> Self {
        let mut tags: HashMap<String, TagPhotos<u8>> = HashMap::new();

        for (slug, tag_photos) in self.tags.iter() {
            tags.insert(slug.to_string(), tag_photos.clone());
        }

        PostLog {
            next_path: self.next_path.clone(),
            prev_path: self.prev_path.clone(),
            happened_on: self.happened_on,
            as_of: self.as_of,
            photo_count: self.photo_count,
            tags,
            files_have_changed: self.files_have_changed,
            cover_photo: if let Some(p) = &self.cover_photo {
                Some(p.clone())
            } else {
                None
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlogLog {
    pub tags: HashMap<String, TagPhotos<PhotoPath>>,
}

impl BlogLog {
    /// Save information about photo tags to avoid unecessary re-processing
    pub fn write(root: &Path, blog: &Blog) {
        let log = BlogLog {
            tags: blog.tags.clone(),
        };
        let path = root.join(LOG_FILE);
        let pretty = PrettyConfig {
            depth_limit: 4,
            ..PrettyConfig::default()
        };

        write_result(&path, || to_string_pretty(&log, pretty));
    }

    pub fn empty() -> BlogLog {
        BlogLog {
            tags: HashMap::new(),
        }
    }

    /// Load log file from path
    pub fn load(path: &Path) -> Option<Self> {
        load_ron(path, LOG_FILE, false)
    }

    /// Whether logged values differ from current blog values. This method also
    /// updates the `changed` field of specific `TagPhotos`.
    pub fn differs(&mut self, blog: &Blog) -> bool {
        for (slug, tag_photos) in &self.tags {
            

            if let Some(tp) = blog.tags.get(slug) {
                if tp.name != tag_photos.name
                    || tp.photos.len() != tag_photos.photos.len()
                {
                    return true;
                }
            } else {
                return true;
            }
        }

        false
    }
}
