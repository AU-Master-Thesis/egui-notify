use egui::{pos2, Pos2, Vec2};

/// Anchor where to show toasts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    /// Top right corner.
    TopRight,
    /// Top left corner.
    TopLeft,
    /// Top center
    TopCenter,
    /// Bottom right corner.
    BottomRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom center
    BottomCenter,
}

impl Anchor {
    #[inline]
    pub(crate) fn anim_side(&self) -> f32 {
        match self {
            Anchor::TopRight | Anchor::BottomRight => 1.,
            Anchor::TopLeft | Anchor::BottomLeft => -1.,
            Anchor::TopCenter | Anchor::BottomCenter => 0.0,
        }
    }
}

impl Anchor {
    pub(crate) fn screen_corner(&self, sc: Pos2, margin: Vec2) -> Pos2 {
        let mut out = match self {
            Anchor::TopRight => pos2(sc.x, 0.),
            Anchor::TopLeft => pos2(0., 0.),
            Anchor::BottomRight => sc,
            Anchor::BottomLeft => pos2(0., sc.y),
            Anchor::TopCenter => pos2(sc.x / 2.0, 0.),
            Anchor::BottomCenter => pos2(sc.x / 2.0, sc.y),
        };
        self.apply_margin(&mut out, margin);
        out
    }

    pub(crate) fn apply_margin(&self, pos: &mut Pos2, margin: Vec2) {
        match self {
            Anchor::TopRight => {
                pos.x -= margin.x;
                pos.y += margin.y;
            }
            Anchor::TopLeft => {
                pos.x += margin.x;
                pos.y += margin.y
            }
            Anchor::BottomRight => {
                pos.x -= margin.x;
                pos.y -= margin.y;
            }
            Anchor::BottomLeft => {
                pos.x += margin.x;
                pos.y -= margin.y;
            }
            Anchor::TopCenter => {
                pos.y += margin.y;
            }
            Anchor::BottomCenter => {
                pos.y -= margin.y;
            }
        }
    }
}
