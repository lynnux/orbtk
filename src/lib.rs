#![crate_type="lib"]

pub use sys::Window;

pub use click::Click;
pub use color::Color;
pub use event::Event;
pub use label::Label;
pub use point::Point;
pub use progress_bar::ProgressBar;
pub use rect::Rect;
pub use renderer::Renderer;
pub use widget::Widget;

pub mod click;
pub mod color;
pub mod event;
pub mod label;
pub mod point;
pub mod progress_bar;
pub mod rect;
pub mod renderer;
pub mod widget;

#[cfg(target_os = "redox")]
#[path="orbital/mod.rs"]
pub mod sys;

#[cfg(not(target_os = "redox"))]
#[path="sdl2/mod.rs"]
pub mod sys;