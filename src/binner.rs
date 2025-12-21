use glam::uvec2;

use crate::Shape;

/// Structure used for shape tile-binning on the CPU side
pub struct ShapeBinner {
    pub tile_size: u32,
    pub resolution: (u32, u32),

    pub shapes_by_tile: Vec<Vec<usize>>,

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
        // Clear each tile individually to avoid reallocations
        self.shapes_by_tile.iter_mut().for_each(|v| v.clear());
        // Resize the outer vector, adding/removing tile vectors where neded
        self.shapes_by_tile
            .resize((tiles_x * tiles_y) as usize, Vec::new());
    }

    pub fn bin_shape(&mut self, shape: &Shape, shape_index: usize) {
        let bounds = shape.bounds();
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
                    tile_shapes.push(shape_index);
                }
            }
        }
    }

    /// Bin shapes into tiles. Clears previous data before binning.
    pub fn calculate_shape_ranges(&mut self) {
        self.tile_ranges.clear();
        self.shape_indices.clear();

        let screen_tiles_x = self.resolution.0.div_ceil(self.tile_size);
        let screen_tiles_y = self.resolution.1.div_ceil(self.tile_size);
        let total_tiles = screen_tiles_x * screen_tiles_y;

        self.tile_ranges.reserve(total_tiles as usize + 1);
        self.tile_ranges.push(0); // First tile starts at index 0
        for tile_shapes in &self.shapes_by_tile {
            self.shape_indices
                .extend(tile_shapes.iter().map(|&i| i as u32));
            self.tile_ranges.push(self.shape_indices.len() as u32);
        }
    }
}
