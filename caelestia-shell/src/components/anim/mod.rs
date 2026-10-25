//! Animation primitives matching `shell/components/`
//! Includes `Anim`, `CAnim`, `AnchorAnim`, and `AnimLoader`.

pub mod anim;
pub mod anchor_anim;
pub mod anim_loader;
pub mod canim;

pub use anchor_anim::{AnchorAnim, AnchorAnimType};
pub use anim::{
    Anim, AnimType, CubicBezier, EASING_DEFAULT_EFFECTS, EASING_DEFAULT_SPATIAL, EASING_EMPHASIZED,
    EASING_FAST_EFFECTS, EASING_FAST_SPATIAL, EASING_SLOW_EFFECTS, EASING_SLOW_SPATIAL, EASING_STANDARD,
};
pub use anim_loader::{AnimLoader, AnimLoaderState, LoaderPhase};
pub use canim::CAnim;
