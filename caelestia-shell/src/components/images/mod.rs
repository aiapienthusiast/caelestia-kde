//! Image and media primitives library
//! 100% parity with `shell/components/images/`.

pub mod animated_image;
pub mod caching_image;
pub mod caching_video;
pub mod fade_image;
pub mod icon_image;
pub mod window_preview;

pub use animated_image::{AnimatedImageController, AnimatedImageView};
pub use caching_image::{CachedImageSource, CachingImageView, ImageFillMode};
pub use caching_video::{CachingVideoView, VideoWallpaperController};
pub use fade_image::{FadeImageController, FadeImageView};
pub use icon_image::{IconImageView, IconSource};
pub use window_preview::{WindowPreviewView, WindowStreamState};
