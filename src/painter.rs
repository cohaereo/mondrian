use crate::{
    binner::ShapeBinner,
    shape::{CornerRadius, Primitive, Shape},
};
use glam::{Vec2, Vec4};

pub struct Painter {
    shapes: Vec<Shape>,
    next_group_id: u32,
    in_group: bool,
    first_shape_in_group: usize,

    binner: ShapeBinner,
    started: bool,
}

impl Painter {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            next_group_id: 0,
            in_group: false,
            first_shape_in_group: 0,

            binner: ShapeBinner::new(32, (0, 0)),
            started: false,
        }
    }

    pub fn add_shape(&mut self, mut shape: Shape) -> &mut Shape {
        shape.group_id = if self.in_group {
            self.next_group_id
        } else {
            self.first_shape_in_group = self.shapes.len();
            self.next_group_id += 1;
            self.next_group_id - 1
        };
        self.shapes.push(shape);
        self.shapes.last_mut().unwrap()
    }

    fn clear_shapes(&mut self) {
        self.shapes.clear();
        self.next_group_id = 0;
        self.in_group = false;
    }

    pub fn start(&mut self, resolution: (u32, u32)) {
        if self.started {
            panic!("Painter::start() called before Painter::finish()");
        }
        self.binner.update_resolution(resolution);
        self.started = true;
    }

    pub fn finish<F: FnOnce(&[Shape], &ShapeBinner)>(&mut self, f: F) {
        if !self.started {
            panic!("Painter::finish() called before Painter::start()");
        }

        self.binner.bin_shapes(&self.shapes);
        f(&self.shapes, &self.binner);
        self.clear_shapes();
        self.started = false;
    }

    /// Begin a group of shapes. All shapes added while in a group will share the same group ID.
    ///
    /// End the group with `end_group()`.
    ///
    /// Calling `begin_group()` while already in a group will start a new group.
    pub fn begin_group(&mut self) {
        if self.in_group {
            self.next_group_id += 1;
        }
        self.in_group = true;
        self.first_shape_in_group = self.shapes.len();
    }

    pub fn end_group(&mut self) {
        self.in_group = false;
        self.next_group_id += 1;
    }
}

impl Default for Painter {
    fn default() -> Self {
        Self::new()
    }
}

// Shape helper methods
impl Painter {
    pub fn add_primitive(&mut self, primitive: Primitive, color: Vec4) -> &mut Shape {
        let shape = Shape {
            primitive,
            color,
            distance_offset: 0.0,
            line_width: 0.0,
            group_id: 0,
            texture_id: None,
        };
        self.add_shape(shape)
    }

    pub fn add_circle(
        &mut self,
        center: impl Into<Vec2>,
        radius: f32,
        color: impl Into<Vec4>,
        line_width: f32,
    ) -> &mut Shape {
        let center = center.into();
        let color = color.into();
        self.add_primitive(Primitive::Circle { center, radius }, color)
            .with_line_width(line_width)
    }

    pub fn add_filled_circle(
        &mut self,
        center: impl Into<Vec2>,
        radius: f32,
        color: impl Into<Vec4>,
    ) -> &mut Shape {
        let center = center.into();
        let color = color.into();
        self.add_primitive(Primitive::Circle { center, radius }, color)
    }

    pub fn add_rect(
        &mut self,
        min: impl Into<Vec2>,
        max: impl Into<Vec2>,
        corner_radius: impl Into<CornerRadius>,
        color: impl Into<Vec4>,
        line_width: f32,
    ) -> &mut Shape {
        let min = min.into();
        let max = max.into();
        let center = (min + max) * 0.5;
        let half_extents = (max - min) * 0.5;
        let corner_radius = corner_radius.into();
        let color = color.into();
        self.add_primitive(
            Primitive::Rect {
                center,
                half_extents,
                corner_radius,
            },
            color,
        )
        .with_line_width(line_width)
    }

    pub fn add_filled_rect(
        &mut self,
        min: impl Into<Vec2>,
        max: impl Into<Vec2>,
        corner_radius: impl Into<CornerRadius>,
        color: impl Into<Vec4>,
    ) -> &mut Shape {
        let min = min.into();
        let max = max.into();
        let center = (min + max) * 0.5;
        let half_extents = (max - min) * 0.5;
        let corner_radius = corner_radius.into();
        let color = color.into();
        self.add_primitive(
            Primitive::Rect {
                center,
                half_extents,
                corner_radius,
            },
            color,
        )
    }

    pub fn add_rect_center_size(
        &mut self,
        center: impl Into<Vec2>,
        half_extents: impl Into<Vec2>,
        corner_radius: impl Into<CornerRadius>,
        color: impl Into<Vec4>,
        line_width: f32,
    ) -> &mut Shape {
        let center = center.into();
        let half_extents = half_extents.into();
        let corner_radius = corner_radius.into();
        let color = color.into();
        self.add_primitive(
            Primitive::Rect {
                center,
                half_extents,
                corner_radius,
            },
            color,
        )
        .with_line_width(line_width)
    }

    pub fn add_filled_rect_center_size(
        &mut self,
        center: impl Into<Vec2>,
        half_extents: impl Into<Vec2>,
        corner_radius: impl Into<CornerRadius>,
        color: impl Into<Vec4>,
    ) -> &mut Shape {
        let center = center.into();
        let half_extents = half_extents.into();
        let corner_radius = corner_radius.into();
        let color = color.into();
        self.add_primitive(
            Primitive::Rect {
                center,
                half_extents,
                corner_radius,
            },
            color,
        )
    }

    pub fn add_line(
        &mut self,
        p1: impl Into<Vec2>,
        p2: impl Into<Vec2>,
        radius: f32,
        color: impl Into<Vec4>,
    ) -> &mut Shape {
        let p1 = p1.into();
        let p2 = p2.into();
        let color = color.into();
        self.add_primitive(Primitive::Line { p1, p2 }, color)
            .with_distance_offset(radius)
    }

    pub fn add_filled_line(
        &mut self,
        p1: impl Into<Vec2>,
        p2: impl Into<Vec2>,
        radius: f32,
        color: impl Into<Vec4>,
    ) -> &mut Shape {
        let p1 = p1.into();
        let p2 = p2.into();
        let color = color.into();
        self.add_primitive(Primitive::Line { p1, p2 }, color)
            .with_distance_offset(-radius)
    }

    pub fn add_triangle(
        &mut self,
        p1: impl Into<Vec2>,
        p2: impl Into<Vec2>,
        p3: impl Into<Vec2>,
        color: impl Into<Vec4>,
        line_width: f32,
    ) -> &mut Shape {
        let p1 = p1.into();
        let p2 = p2.into();
        let p3 = p3.into();
        let color = color.into();
        self.add_primitive(Primitive::Triangle { p1, p2, p3 }, color)
            .with_line_width(line_width)
    }

    pub fn add_filled_triangle(
        &mut self,
        p1: impl Into<Vec2>,
        p2: impl Into<Vec2>,
        p3: impl Into<Vec2>,
        color: impl Into<Vec4>,
    ) -> &mut Shape {
        let p1 = p1.into();
        let p2 = p2.into();
        let p3 = p3.into();
        let color = color.into();
        self.add_primitive(Primitive::Triangle { p1, p2, p3 }, color)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_circle_sector(
        &mut self,
        center: impl Into<Vec2>,
        radius_inner: impl Into<f32>,
        radius_outer: impl Into<f32>,
        angle_start: impl Into<f32>,
        angle_end: impl Into<f32>,
        color: impl Into<Vec4>,
        line_width: f32,
    ) -> &mut Shape {
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
        )
        .with_line_width(line_width)
    }

    pub fn add_filled_circle_sector(
        &mut self,
        center: impl Into<Vec2>,
        radius_inner: impl Into<f32>,
        radius_outer: impl Into<f32>,
        angle_start: impl Into<f32>,
        angle_end: impl Into<f32>,
        color: impl Into<Vec4>,
    ) -> &mut Shape {
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
        )
    }

    pub fn add_polyquad(
        &mut self,
        p1: impl Into<Vec2>,
        p2: impl Into<Vec2>,
        p3: impl Into<Vec2>,
        p4: impl Into<Vec2>,
        color: impl Into<Vec4>,
        line_width: f32,
    ) -> &mut Shape {
        let points = [p1.into(), p2.into(), p3.into(), p4.into()];
        let color = color.into();
        self.add_primitive(Primitive::PolyQuad { points }, color)
            .with_line_width(line_width)
    }

    pub fn add_filled_polyquad(
        &mut self,
        p1: impl Into<Vec2>,
        p2: impl Into<Vec2>,
        p3: impl Into<Vec2>,
        p4: impl Into<Vec2>,
        color: impl Into<Vec4>,
    ) -> &mut Shape {
        let points = [p1.into(), p2.into(), p3.into(), p4.into()];
        let color = color.into();
        self.add_primitive(Primitive::PolyQuad { points }, color)
    }
}
