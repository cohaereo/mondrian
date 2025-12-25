use assert_offset::AssertOffsets;
use glam::Vec4;

use crate::{
    Primitive, Shape,
    shape::{BoundingBox, ShapeFlags},
};

#[repr(C)]
#[derive(Clone, Copy, AssertOffsets, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuShape {
    #[offset(0x00)]
    pub header: ShapeHeader,
    pub distance_offset: f32,
    pub line_width: f32,
    pub flags: GpuShapeFlags,

    #[offset(0x10)]
    pub bounds: BoundingBox,

    #[offset(0x20)]
    pub color: Vec4,

    #[offset(0x30)]
    pub glow: Vec4,

    #[offset(0x40)]
    pub params: GpuShapeParams,
}

impl GpuShape {
    pub fn from_shape(shape: &Shape, texture_id: Option<u32>) -> Self {
        let shape_type = match shape.primitive {
            Primitive::Circle { .. } => 0,
            Primitive::Triangle { .. } => 1,
            Primitive::Rect { .. } => 2,
            Primitive::Line { .. } => 3,
            Primitive::CircleSector { .. } => 4,
            Primitive::PolyQuad { .. } => 5,
        };

        let bounds = shape.bounds();

        Self {
            header: ShapeHeader::new(shape_type, texture_id),
            distance_offset: shape.distance_offset,
            line_width: shape.line_width,
            flags: GpuShapeFlags::new(shape.flags, shape.group_id),
            bounds,
            color: shape.color,
            glow: shape.glow,
            params: GpuShapeParams::from(&shape.primitive),
        }
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShapeHeader(u32);

impl ShapeHeader {
    pub const SENTINEL: Self = ShapeHeader(u32::MAX);

    pub fn new(shape_type: u32, texture_id: Option<u32>) -> Self {
        let texture_id_bits = texture_id.unwrap_or(u32::MAX) << 8;
        let shape_type_bits = shape_type & 0xFF;
        ShapeHeader(texture_id_bits | shape_type_bits)
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuShapeFlags(u32);

impl GpuShapeFlags {
    pub fn new(flags: ShapeFlags, group_id: u32) -> Self {
        let flags_bits = (flags.bits() as u32) << 24;
        let group_id_bits = group_id & 0x00FF_FFFF;
        GpuShapeFlags(flags_bits | group_id_bits)
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuShapeParams {
    params: [f32; 8],
}

impl From<&Primitive> for GpuShapeParams {
    fn from(value: &Primitive) -> Self {
        match *value {
            Primitive::Circle { center, radius } => [center.x, center.y, radius].into(),
            Primitive::Triangle { p1, p2, p3 } => [p1.x, p1.y, p2.x, p2.y, p3.x, p3.y].into(),
            Primitive::Rect {
                center,
                half_extents: extents,
                corner_radius,
            } => [
                center.x,
                center.y,
                extents.x,
                extents.y,
                corner_radius.top_left,
                corner_radius.top_right,
                corner_radius.bottom_right,
                corner_radius.bottom_left,
            ]
            .into(),
            Primitive::Line { p1, p2 } => [p1.x, p1.y, p2.x, p2.y].into(),
            Primitive::CircleSector {
                center,
                radius_inner,
                radius_outer,
                angle_start,
                angle_end,
            } => [
                center.x,
                center.y,
                radius_inner,
                radius_outer,
                angle_start,
                angle_end,
            ]
            .into(),
            Primitive::PolyQuad { points } => [
                points[0].x,
                points[0].y,
                points[1].x,
                points[1].y,
                points[2].x,
                points[2].y,
                points[3].x,
                points[3].y,
            ]
            .into(),
        }
    }
}

macro_rules! impl_from_arrays {
    ($($len:expr),*) => {
        $(
            impl From<[f32; $len]> for GpuShapeParams {
                fn from(arr: [f32; $len]) -> Self {
                    let mut params = [0.0; 8];
                    params[..$len].copy_from_slice(&arr);
                    GpuShapeParams { params }
                }
            }
        )*
    };
}

impl_from_arrays!(1, 2, 3, 4, 5, 6, 7, 8);
