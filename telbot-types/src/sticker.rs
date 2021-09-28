use crate::file::PhotoSize;
use serde::{Deserialize, Serialize};

/// This object represents a sticker.
#[derive(Deserialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Sticker width
    pub width: u32,
    /// Sticker height
    pub height: u32,
    /// *True*, if the sticker is [animated](https://telegram.org/blog/animated-stickers)
    pub is_animated: bool,
    /// Sticker thumbnail in the .WEBP or .JPG format
    pub thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// File size
    pub file_size: Option<u32>,
}

/// This object describes the position on faces where a mask should be placed by default.
#[derive(Deserialize)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed.
    /// One of “forehead”, “eyes”, “mouth”, or “chin”.
    pub point: MaskPoint,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right.
    /// For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: f32,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom.
    /// For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: f32,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f32,
}

/// The part of the face used in masked stickers.
#[derive(Deserialize)]
pub enum MaskPoint {
    Forehead,
    Eyes,
    Mouth,
    Chin,
}
