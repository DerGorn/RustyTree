use std::{
    collections::{hash_set::Iter, HashSet},
    hash::Hash,
    rc::Rc,
};

use crate::{math_2d::Vector, PhysicalSize};
pub struct SpatialHashgrid<T>
where
    T: Eq + Hash,
{
    grid_size: PhysicalSize<f64>,
    cell_size: PhysicalSize<u32>,
    grid: Vec<HashSet<Rc<T>>>,
}
impl<T> SpatialHashgrid<T>
where
    T: Eq + Hash,
{
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
            .min(self.grid_size.width);
        let y = (position.y / self.cell_size.height as f64)
            .ceil()
            .min(self.grid_size.height);
        (x * y) as usize - 1
    }

    ///Adds the element to the set corresponding to position.
    ///
    ///Returns whether the value was newly inserted. That is:
    ///
    ///If the set did not previously contain this value, true is returned.
    ///If the set already contained this value, false is returned.
    pub fn insert(&mut self, element: Rc<T>, position: &Vector) -> bool {
        let index = self.spatial_hash(&position);
        self.grid.get_mut(index).unwrap().insert(element)
    }

    ///Removes the element from the set corresponding to position. Returns whether the element was present in the set.
    pub fn remove(&mut self, element: &T, position: &Vector) -> bool {
        let index = self.spatial_hash(&position);
        self.grid.get_mut(index).unwrap().remove(element)
    }

    pub fn get_cell(&self, position: &Vector) -> Iter<'_, Rc<T>> {
        let index = self.spatial_hash(&position);
        self.grid.get(index).unwrap().iter()
    }
}
