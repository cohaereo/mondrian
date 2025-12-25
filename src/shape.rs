use bitflags::bitflags;
use glam::{Vec2, Vec4};

slotmap::new_key_type! {
    pub struct TextureId;
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub primitive: Primitive,
    pub color: Vec4,
    /// A value added to the distance field before rendering. Negative values
    /// will make the shape appear larger, positive values will make it appear smaller.
    pub distance_offset: f32,
    /// When non-zero, indicates that the shape should be rendered as a line
    /// with the given width instead of a filled shape.
    pub line_width: f32,
    /// An optional group ID for the shape. SDF shapes with the same group ID are joined together
    pub group_id: u32,
    /// An optional texture ID for the shape. If set, the shape will be sample from the given texture, using the SDF as a clip mask.
    /// The shape's color will be multiplied with the texture color.
    pub texture_id: Option<TextureId>,
    pub flags: ShapeFlags,
}

impl Shape {
    /// Calculates the axis-aligned bounding box of the shape, taking into account distance offset, line width, etc.
    ///
    /// This is a conservative estimate, as it is primarily used for culling and tile binning. As such, it may be (slightly) larger than the actual bounds.
    pub fn culling_bounds(&self) -> BoundingBox {
        // cohae: Right now this is the same as bounds(), but in the future, glow/shadows will require culling bounds to be larger than the actual shape bounds.
        let mut bounds = self.primitive.bounds();
        bounds.grow(-self.distance_offset);
        bounds.grow(self.line_width * 0.5);
        bounds
    }

    /// Calculates the axis-aligned bounding box of the primitive, without considering distance offset, line width, etc.
    ///
    /// This is a tight fit around the primitive and it's outline. This can be used for precise calculations where the exact shape bounds are needed.
    pub fn bounds(&self) -> BoundingBox {
        let mut bounds = self.primitive.bounds();
        bounds.grow(-self.distance_offset);
        bounds.grow(self.line_width * 0.5);
        bounds
    }

    pub fn with_distance_offset(&mut self, offset: f32) -> &mut Self {
        self.distance_offset = offset;
        self
    }

    pub fn with_line_width(&mut self, line_width: f32) -> &mut Self {
        self.line_width = line_width;
        self
    }

    pub fn with_texture_id(&mut self, texture_id: TextureId) -> &mut Self {
        self.texture_id = Some(texture_id);
        self
    }

    pub fn with_flags(&mut self, flags: ShapeFlags) -> &mut Self {
        self.flags = flags;
        self
    }

    pub fn with_texture_is_sdf(&mut self) -> &mut Self {
        self.flags.insert(ShapeFlags::TEXTURE_SDF);
        self
    }

    pub fn with_texture_is_mtsdf(&mut self) -> &mut Self {
        self.flags.insert(ShapeFlags::TEXTURE_MTSDF);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Primitive {
    Circle {
        center: Vec2,
        radius: f32,
    },
    Triangle {
        p1: Vec2,
        p2: Vec2,
        p3: Vec2,
    },
    Rect {
        center: Vec2,
        half_extents: Vec2,
        corner_radius: CornerRadius,
    },
    Line {
        p1: Vec2,
        p2: Vec2,
    },
    CircleSector {
        center: Vec2,
        radius_inner: f32,
        radius_outer: f32,
        angle_start: f32,
        angle_end: f32,
    },
    PolyQuad {
        points: [Vec2; 4],
    },
}

impl Primitive {
    pub fn bounds(&self) -> BoundingBox {
        match *self {
            Primitive::Circle { center, radius } => BoundingBox {
                min: center - Vec2::splat(radius),
                max: center + Vec2::splat(radius),
            },
            Primitive::Triangle { p1, p2, p3 } => {
                let min = p1.min(p2).min(p3);
                let max = p1.max(p2).max(p3);
                BoundingBox { min, max }
            }
            Primitive::Rect {
                center,
                half_extents: extents,
                ..
            } => BoundingBox {
                min: center - extents,
                max: center + extents,
            },
            Primitive::Line { p1, p2 } => {
                let min = p1.min(p2);
                let max = p1.max(p2);
                BoundingBox { min, max }
            }
            Primitive::CircleSector {
                center,
                radius_outer,
                ..
            } => BoundingBox {
                min: center - Vec2::splat(radius_outer),
                max: center + Vec2::splat(radius_outer),
            },
            Primitive::PolyQuad { points } => {
                let mut min = points[0];
                let mut max = points[0];
                for &point in &points[1..] {
                    min = min.min(point);
                    max = max.max(point);
                }
                BoundingBox { min, max }
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CornerRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl From<f32> for CornerRadius {
    fn from(val: f32) -> Self {
        CornerRadius {
            top_left: val,
            top_right: val,
            bottom_right: val,
            bottom_left: val,
        }
    }
}

impl From<[f32; 4]> for CornerRadius {
    fn from(vals: [f32; 4]) -> Self {
        CornerRadius {
            top_left: vals[0],
            top_right: vals[1],
            bottom_right: vals[2],
            bottom_left: vals[3],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BoundingBox {
    pub min: Vec2,
    pub max: Vec2,
}

impl BoundingBox {
    pub const EMPTY: Self = BoundingBox {
        min: Vec2::splat(f32::MAX),
        max: Vec2::splat(f32::MIN),
    };
    pub const INFINITE: Self = BoundingBox {
        min: Vec2::splat(f32::MIN),
        max: Vec2::splat(f32::MAX),
    };

    pub fn grow(&mut self, amount: f32) {
        self.min -= Vec2::splat(amount);
        self.max += Vec2::splat(amount);
    }

    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self::EMPTY
    }
}

bitflags! {
    #[derive(Default, Clone, Copy, Debug)]
    pub struct ShapeFlags: u8 {
        const TEXTURE_SDF = 1 << 0;
        const TEXTURE_MTSDF = 1 << 1;
    }
}
