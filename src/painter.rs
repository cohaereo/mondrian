use crate::shape::{CornerRadius, Primitive, Shape};
use glam::{Vec2, Vec4};

pub struct Painter {
    shapes: Vec<Shape>,
}

impl Painter {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    fn clear_shapes(&mut self) {
        self.shapes.clear();
    }

    pub fn finish<F: FnOnce(&[Shape])>(&mut self, f: F) {
        f(&self.shapes);
        self.clear_shapes();
    }
}

impl Default for Painter {
    fn default() -> Self {
        Self::new()
    }
}

// Shape helper methods
impl Painter {
    pub fn add_primitive(&mut self, primitive: Primitive, color: Vec4, distance_offset: f32) {
        let shape = Shape {
            primitive,
            color,
            distance_offset,
        };
        self.add_shape(shape);
    }

    pub fn add_circle(&mut self, center: impl Into<Vec2>, radius: f32, color: impl Into<Vec4>) {
        let center = center.into();
        let color = color.into();
        self.add_primitive(Primitive::Circle { center, radius }, color, 0.0);
    }

    pub fn add_box(
        &mut self,
        center: impl Into<Vec2>,
        extents: impl Into<Vec2>,
        corner_radius: impl Into<CornerRadius>,
        color: impl Into<Vec4>,
    ) {
        let center = center.into();
        let extents = extents.into();
        let corner_radius = corner_radius.into();
        let color = color.into();
        self.add_primitive(
            Primitive::Box {
                center,
                extents,
                corner_radius,
            },
            color,
            0.0,
        );
    }

    pub fn add_segment(
        &mut self,
        p1: impl Into<Vec2>,
        p2: impl Into<Vec2>,
        radius: f32,
        color: impl Into<Vec4>,
    ) {
        let p1 = p1.into();
        let p2 = p2.into();
        let color = color.into();
        self.add_primitive(Primitive::Segment { p1, p2 }, color, -radius);
    }

    pub fn add_triangle(
        &mut self,
        p1: impl Into<Vec2>,
        p2: impl Into<Vec2>,
        p3: impl Into<Vec2>,
        color: impl Into<Vec4>,
    ) {
        let p1 = p1.into();
        let p2 = p2.into();
        let p3 = p3.into();
        let color = color.into();
        self.add_primitive(Primitive::Triangle { p1, p2, p3 }, color, 0.0);
    }

    pub fn add_circle_sector(
        &mut self,
        center: impl Into<Vec2>,
        radius_inner: impl Into<f32>,
        radius_outer: impl Into<f32>,
        angle_start: impl Into<f32>,
        angle_end: impl Into<f32>,
        color: impl Into<Vec4>,
    ) {
        let center = center.into();
        let radius_inner = radius_inner.into();
        let radius_outer = radius_outer.into();
        let angle_start = angle_start.into();
        let angle_end = angle_end.into();
        let color = color.into();
        self.add_primitive(
            Primitive::CircleSector {
                center,
                radius_inner,
                radius_outer,
                angle_start,
                angle_end,
            },
            color,
            0.0,
        );
    }

    pub fn add_polyquad(
        &mut self,
        p1: impl Into<Vec2>,
        p2: impl Into<Vec2>,
        p3: impl Into<Vec2>,
        p4: impl Into<Vec2>,
        color: impl Into<Vec4>,
    ) {
        let points = [p1.into(), p2.into(), p3.into(), p4.into()];
        let color = color.into();
        self.add_primitive(Primitive::PolyQuad { points }, color, 0.0);
    }
}
