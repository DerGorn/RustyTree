use std::{
    collections::{hash_set::Iter, HashSet},
    hash::Hash,
};

use crate::{math_2d::Vector, PhysicalSize};

#[derive(Debug)]
pub struct SpatialHashgrid<T>
where
    T: Eq + Hash + Clone,
{
    ///number of cells in the grid
    grid_size: PhysicalSize<f64>,
    ///dimensions of a cell
    cell_size: PhysicalSize<u32>,
    grid: Vec<HashSet<T>>,
}
impl<T> SpatialHashgrid<T>
where
    T: Eq + Hash + Clone,
{
    ///Creates a SpatialHashgrid from the dimensions of the grid `grid_size` and the dimensions of each cell `cell_size`
    pub fn new(grid_size: PhysicalSize<u32>, cell_size: PhysicalSize<u32>) -> Self {
        let grid_size = PhysicalSize::new(
            (grid_size.width as f64 / cell_size.width as f64).ceil(),
            (grid_size.height as f64 / cell_size.height as f64).ceil(),
        );
        let amount_of_cells = grid_size.width * grid_size.height;
        Self {
            grid_size,
            cell_size,
            grid: vec![HashSet::new(); amount_of_cells as usize],
        }
    }

    fn spatial_hash(&self, position: &Vector) -> usize {
        let x = (position.x / self.cell_size.width as f64)
            .ceil()
            .min(self.grid_size.width - 1.0);
        let y = (position.y / self.cell_size.height as f64)
            .ceil()
            .min(self.grid_size.height - 1.0);
        ((x + y * self.grid_size.width) as usize).min(self.grid.len() - 1)
    }

    ///Adds the element to the set corresponding to position.
    ///
    ///Returns whether the value was newly inserted. That is:
    ///
    ///If the set did not previously contain this value, true is returned.
    ///If the set already contained this value, false is returned.
    pub fn insert(&mut self, element: T, position: &Vector) -> bool {
        let index = self.spatial_hash(&position);
        self.grid.get_mut(index).unwrap().insert(element)
    }

    ///Removes the element from the set corresponding to position. Returns whether the element was present in the set.
    pub fn remove(&mut self, element: &T, position: &Vector) -> bool {
        let index = self.spatial_hash(&position);
        self.grid.get_mut(index).unwrap().remove(element)
    }

    ///Returns true if the set contains a value.
    pub fn contains(&mut self, element: &T, position: &Vector) -> bool {
        let index = self.spatial_hash(&position);
        self.grid.get_mut(index).unwrap().contains(element)
    }

    pub fn get_cell(&self, position: &Vector) -> Iter<'_, T> {
        let index = self.spatial_hash(&position);
        self.grid.get(index).unwrap().iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::Res;
    use std::fmt::Write;

    use super::*;

    #[test]
    fn spatial_hash() {
        let mut grid = SpatialHashgrid::new(PhysicalSize::new(2, 2), PhysicalSize::new(1, 1));

        for x in 0..=2 {
            for y in 0..=2 {
                grid.insert(format!("({}, {})", x, y), &Vector::new(x as f64, y as f64));
            }
        }

        assert_eq!(grid.grid[0].len(), 1);
        assert_eq!(grid.grid[1].len(), 2);
        assert_eq!(grid.grid[2].len(), 2);
        assert_eq!(grid.grid[3].len(), 4);
    }

    #[test]
    fn get_cell() -> Res<()> {
        let mut grid = SpatialHashgrid::new(PhysicalSize::new(2, 2), PhysicalSize::new(1, 1));

        let el = format!("({}, {})", 0, 0);
        let pos = Vector::new(0.0_f64, 0.0_f64);

        grid.insert(el.clone(), &pos);

        assert!(grid.contains(&el, &pos));

        let cell: Vec<&String> = grid.get_cell(&pos).collect();
        assert!(cell.contains(&&el));

        let mut f = String::new();
        write!(f, "{:?}", cell)?;

        let expected = "[\"(0, 0)\"]";
        assert_eq!(expected, f);

        Ok(())
    }
}
