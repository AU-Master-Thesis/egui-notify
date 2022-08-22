use crate::{Anchor, TOAST_HEIGHT, TOAST_WIDTH};
use egui::{Rect, Pos2, pos2, vec2};

/// Level of importance
#[derive(Default)]
#[allow(missing_docs)]
pub enum ToastLevel {
    #[default]
    Info,
    Warning,
    Error
}

// Life is too short to write docs
impl ToastLevel {
    #[inline]
    pub(crate) fn is_info(&self) -> bool {
        matches!(self, Self::Info)
    }
    
    #[inline]
    pub(crate) fn is_warning(&self) -> bool {
        matches!(self, Self::Warning)
    }

    #[inline]
    pub(crate) fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

pub(crate) enum ToastState {
    Appear,
    Idle,
}

impl ToastState {
    pub fn appearing(&self) -> bool {
        matches!(self, Self::Appear)
    }

    pub fn idling(&self) -> bool {
        matches!(self, Self::Idle)
    }
}

/// Single notification or *toast*
pub struct Toast {
    pub(crate) level: ToastLevel,
    pub(crate) caption: String,
    pub(crate) duration: Option<f32>,
    pub(crate) initial_duration: Option<f32>,
    pub(crate) height: f32,
    pub(crate) width: f32,
    pub(crate) closable: bool,

    pub(crate) state: ToastState,
    pub(crate) value: f32,
}

impl Toast {
    fn new(caption: impl Into<String>, level: ToastLevel) -> Self {
        Self {
            initial_duration: Some(5.),
            caption: caption.into(),
            height: TOAST_HEIGHT,
            width: TOAST_WIDTH,
            duration: Some(5.),
            closable: true,
            level,

            
            value: 0.,
            state: ToastState::Appear,
        }
    }

    /// Creates new info toast, can be closed by default.
    pub fn info(caption: impl Into<String>) -> Self {
        Self::new(caption, ToastLevel::Info)
    }

    /// Creates new warning toast, can be closed by default.
    pub fn warning(caption: impl Into<String>) -> Self {
        Self::new(caption, ToastLevel::Warning)
    }

    /// Creates new error toast, can not be closed by default.
    pub fn error(caption: impl Into<String>) -> Self {
        Self::new(caption, ToastLevel::Error)
            .closable(false)
    }

    /// Can use close the toast?
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// In what time should the toast expire?
    pub fn with_duration(mut self, seconds: f32) -> Self {
        self.initial_duration = Some(seconds);
        self.duration = Some(seconds);
        self
    }

    /// Toast's box height
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Toast's box width
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub(crate) fn calc_anchored_rect(&self, pos: Pos2, anchor: Anchor) -> Rect {
        match anchor {
            Anchor::TopRight => {
                Rect {
                    min: pos2(pos.x - self.width, pos.y),
                    max: pos2(pos.x, pos.y + self.height),
                }
            },
            Anchor::TopLeft => Rect {
                min: pos,
                max: pos + vec2(self.width, self.height),
            },
            Anchor::BottomRight => Rect {
                min: pos - vec2(self.width, self.height),
                max: pos,
            },
            Anchor::BottomLeft => Rect {
                min: pos2(pos.x, pos.y - self.height),
                max: pos2(pos.x + self.width, pos.y),
            },
        }
    }

    pub(crate) fn adjust_next_pos(&self, pos: &mut Pos2, anchor: Anchor, vertical: bool, spacing: f32) {
        match anchor {
            Anchor::TopRight | Anchor::TopLeft if vertical => pos.y += self.height + spacing,
            Anchor::TopRight if !vertical => pos.x -= self.width + spacing,
            Anchor::TopLeft if !vertical => pos.x += self.width + spacing,

            Anchor::BottomRight | Anchor::BottomLeft if vertical => pos.y -= self.height + spacing,
            Anchor::BottomRight if !vertical => pos.x -= self.width + spacing,
            Anchor::BottomLeft if !vertical => pos.x += self.width + spacing,
            _ => unreachable!()
        }
    }
}