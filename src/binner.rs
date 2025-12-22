use glam::uvec2;

use crate::{Shape, shape::BoundingBox};

/// Structure used for shape tile-binning on the CPU side
pub struct ShapeBinner {
    pub tile_size: u32,
    pub resolution: (u32, u32),

    pub shapes_by_tile: Vec<Vec<u32>>,

    pub tile_ranges: Vec<u32>,
    pub shape_indices: Vec<u32>,
}

impl ShapeBinner {
    pub fn new(tile_size: u32, resolution: (u32, u32)) -> Self {
        Self {
            tile_size,
            resolution,
            shapes_by_tile: Vec::new(),
            tile_ranges: Vec::new(),
            shape_indices: Vec::new(),
        }
    }

    pub(crate) fn update_resolution(&mut self, resolution: (u32, u32)) {
        self.resolution = resolution;
        let tiles_x = resolution.0.div_ceil(self.tile_size);
        let tiles_y = resolution.1.div_ceil(self.tile_size);
        // Resize the outer vector, adding/removing tile vectors where neded
        self.shapes_by_tile
            .resize((tiles_x * tiles_y) as usize, Vec::new());
    }

    pub fn bin_shapes(&mut self, shapes: &[Shape]) {
        self.shapes_by_tile.iter_mut().for_each(|v| v.clear());

        let mut current_group_id = 0;
        let mut group_start_index = 0;
        for (i, _shape) in shapes.iter().enumerate() {
            let next_group_id = shapes.get(i + 1).map(|s| s.group_id);
            if next_group_id != Some(current_group_id) {
                let shape_range = group_start_index as u32..(i as u32 + 1);
                let bounds = shapes[group_start_index..=i]
                    .iter()
                    .fold(None, |acc: Option<BoundingBox>, s| {
                        let shape_bounds = s.culling_bounds();
                        Some(if let Some(acc_bounds) = acc {
                            BoundingBox {
                                min: acc_bounds.min.min(shape_bounds.min),
                                max: acc_bounds.max.max(shape_bounds.max),
                            }
                        } else {
                            shape_bounds
                        })
                    })
                    .unwrap();
                self.bin_shape_group(&bounds, shape_range);

                group_start_index = i + 1;
                current_group_id = next_group_id.unwrap_or(0);
            }
        }

        self.calculate_shape_ranges();
    }

    fn bin_shape_group(&mut self, bounds: &BoundingBox, shape_indices: std::ops::Range<u32>) {
        let start_tile = uvec2(
            (bounds.min.x / self.tile_size as f32).floor() as u32,
            (bounds.min.y / self.tile_size as f32).floor() as u32,
        );
        let end_tile = uvec2(
            (bounds.max.x / self.tile_size as f32).floor() as u32,
            (bounds.max.y / self.tile_size as f32).floor() as u32,
        );
        let num_tiles_x = end_tile.x - start_tile.x + 1;
        let num_tiles_y = end_tile.y - start_tile.y + 1;
        let screen_tiles_x = self.resolution.0.div_ceil(self.tile_size);

        for tile_y in start_tile.y..start_tile.y + num_tiles_y {
            for tile_x in start_tile.x..start_tile.x + num_tiles_x {
                let tile_index = (tile_y * screen_tiles_x + tile_x) as usize;
                if let Some(tile_shapes) = self.shapes_by_tile.get_mut(tile_index) {
                    tile_shapes.extend(shape_indices.clone());
                }
            }
        }
    }

    /// Bin shapes into tiles. Clears previous data before binning.
    fn calculate_shape_ranges(&mut self) {
        self.tile_ranges.clear();
        self.shape_indices.clear();

        let screen_tiles_x = self.resolution.0.div_ceil(self.tile_size);
        let screen_tiles_y = self.resolution.1.div_ceil(self.tile_size);
        let total_tiles = screen_tiles_x * screen_tiles_y;

        self.tile_ranges.reserve(total_tiles as usize + 1);
        self.tile_ranges.push(0); // First tile starts at index 0
        for tile_shapes in &self.shapes_by_tile {
            self.shape_indices.extend(tile_shapes.iter());
            self.tile_ranges.push(self.shape_indices.len() as u32);
        }
    }
}
