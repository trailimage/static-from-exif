use crate::{
    config::{BlogConfig, PhotoConfig, PostLog},
    json_ld,
    models::{Category, Photo, TagPhotos},
};
use chrono::{DateTime, FixedOffset};
use core::cmp::Ordering;
use hashbrown::HashMap;
use serde_json;

#[derive(Debug)]
pub struct Post {
    /// File path to the post
    ///
    /// *example* `brother-ride/2.trying-to-survive`
    pub path: String,

    /// Portion of path that is common among series members. For example, with
    /// `brother-ride/2.trying-to-survive` the `series_path` is `brother-ride`.
    pub series_path: String,

    /// Portion of path that is unique among series members. For example, with
    /// `brother-ride/2.trying-to-survive` the `parth_path` is
    /// `2.trying-to-survive`.
    pub part_path: String,

    /// When the depicted events happened
    pub happened_on: Option<DateTime<FixedOffset>>,

    /// When the post was created
    //pub created_on: DateTime<FixedOffset>,
    /// When the post was last updated
    //pub updated_on: DateTime<FixedOffset>,

    /// Title of the post. For series, this will be the series title and the
    /// configured post title will become the `sub_title`.
    pub title: String,

    /// Subtitle of the post. For series, this will be the title the post was
    /// configured with while the post's `title` will be series title.
    pub sub_title: String,

    //pub original_title: String,
    pub summary: String,

    /// Whether post pictures occurred sequentially in a specific time range as
    /// opposed to, for example, a themed set of images from various times
    pub chronological: bool,

    /// Whether post is featured in main navigation (implies not chronological)
    pub featured: bool,

    /// Photos found for the post. If post data were loaded from a previous
    /// render log then this will be empty.
    pub photos: Vec<Photo>,

    /// Next chronological post path (newer)
    pub next_path: Option<String>,
    /// Previous chronological post path (older)
    pub prev_path: Option<String>,

    /// One-based position of this post in a series or 0 if it's not in a series
    pub part: u8,
    /// Whether post is part of a series
    pub is_partial: bool,
    /// Whether next post is part of the same series
    pub next_is_part: bool,
    /// Whether previous post is part of the same series
    pub prev_is_part: bool,
    /// Total number of posts in the series
    pub total_parts: u8,
    /// Whether GPX track was found for the post
    pub has_track: bool,
    /// Categories to which this post belongs
    pub categories: Vec<Category>,

    pub photo_count: usize,

    /// If post photos or configuration have changed, or an adjacent post has
    /// changed, then the post should be re-rendered.
    ///
    /// Posts that haven't changed don't re-parse their photos so `photos` is
    /// an empty vector.
    pub needs_render: bool,

    /// Zero-based index of cover photo within vector of photos
    pub cover_photo_index: usize,

    pub tags: HashMap<String, TagPhotos<u8>>,

    /// Information about previous post photos and configuration
    pub history: PostLog,
}

impl Post {
    /// First photo flagged as `primary`
    pub fn cover_photo(&self) -> Option<&Photo> {
        self.photos.get(self.cover_photo_index)
    }
}

impl Default for Post {
    fn default() -> Self {
        Post {
            path: String::new(),
            series_path: String::new(),
            part_path: String::new(),

            happened_on: None,
            //created_on: min_date(),
            //updated_on: min_date(),
            title: String::new(),
            sub_title: String::new(),
            //original_title: "",
            summary: String::new(),

            chronological: true,
            featured: false,
            photos: Vec::new(),

            next_path: None,
            prev_path: None,

            part: 0,
            total_parts: 0,
            is_partial: false,
            next_is_part: false,
            prev_is_part: false,

            has_track: false,
            categories: Vec::new(),

            photo_count: 0,
            cover_photo_index: 0,
            needs_render: true,

            tags: HashMap::new(),
            history: PostLog::empty(),
        }
    }
}

impl Post {
    pub fn json_ld(&self, config: &BlogConfig) -> serde_json::Value {
        let image = self.cover_photo().and_then(|p| Some(p.json_ld()));
        let categories: Vec<String> = self
            .categories
            .iter()
            .map(|c: &Category| c.name.clone())
            .collect();

        // TODO: implement dates
        serde_json::json!({
            "@type": "BlogPosting",
            "@context": json_ld::CONTEXT,
            "author": json_ld::owner(config),
            "name": &self.title,
            "headline": &self.title,
            "description": &self.summary,
            "image": image,
            "publisher": json_ld::organization(config),
            "mainEntityOfPage": json_ld::web_page(config, "about"),
            "datePublished": "",
            "dateModified": "",
            "articleSection": categories.join(",")
        })
    }

    /// Build root-relative URLs for all post photo sizes
    pub fn build_photo_urls(&mut self, config: &PhotoConfig) {
        for p in self.photos.iter_mut() {
            p.size.build_urls(&self.path, p.index, config);
        }
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.happened_on.partial_cmp(&other.happened_on)
    }
}

impl Ord for Post {
    fn cmp(&self, other: &Post) -> Ordering {
        self.happened_on.cmp(&other.happened_on)
    }
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Post {}
