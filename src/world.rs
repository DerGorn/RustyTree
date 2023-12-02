use crate::{
    math_2d::Vector, physics_2d::Body, physics_2d::CollisionLayer, physics_2d::RefBody,
    renderer::Renderer, PhysicalSize, Res,
};

pub struct CollisionSpecifier {
    collision_layer: usize,
    is_collision_obstacle: Option<bool>,
}

pub struct World {
    bodies: Vec<RefBody>,
    pub renderer: Renderer,
    collision_layers: Vec<CollisionLayer>,
}
impl World {
    /// Creates a new World on the `renderer`. The CollisionLayers will use a SpatialHashGrid with `collision_grid_size` cells in the grid
    pub fn new(
        renderer: Renderer,
        collision_grid_size: PhysicalSize<u32>,
        number_of_collision_layers: usize,
    ) -> Self {
        let grid_size = PhysicalSize::new(renderer.get_width(), renderer.get_height());
        let cell_size = PhysicalSize::new(
            grid_size.width / collision_grid_size.width,
            grid_size.height / collision_grid_size.height,
        );
        let mut collision_layers = Vec::with_capacity(number_of_collision_layers);
        for _ in 0..number_of_collision_layers {
            collision_layers.push(CollisionLayer::new(grid_size, cell_size))
        }
        World {
            renderer,
            bodies: vec![],
            collision_layers,
        }
    }

    pub fn render(&mut self) -> Res<()> {
        self.renderer.clear();
        for body in &self.bodies {
            body.borrow().render(&mut self.renderer);
        }
        self.renderer.render()
    }

    pub fn add_body(
        &mut self,
        body: Body<Vector>,
        collision_specifier: Option<CollisionSpecifier>,
    ) {
        let has_collision = body.has_collision();
        let body: RefBody = body.into();
        self.bodies.push(body.clone());
        if has_collision {
            if let Some(specifier) = collision_specifier {
                self.collision_layers[specifier.collision_layer]
                    .add_body(body, specifier.is_collision_obstacle.unwrap());
            }
        };
    }

    pub fn remove_body(&mut self, body: &RefBody, collision_specifier: Option<CollisionSpecifier>) {
        let has_collision = body.has_collision();
        if has_collision {
            if let Some(specifier) = collision_specifier {
                self.collision_layers[specifier.collision_layer]
                    .remove_body(body, specifier.is_collision_obstacle);
            } else {
                for layer in &mut self.collision_layers {
                    if layer.remove_body(body, None) {
                        break;
                    }
                }
            }
        }
        match self.bodies.iter().position(|el| el == body) {
            None => {}
            Some(index) => {
                self.bodies.remove(index);
            }
        }
    }
}
