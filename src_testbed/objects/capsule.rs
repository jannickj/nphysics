use crate::objects::node::{self, GraphicsNode};
use kiss3d::window;
use na::{Point3, RealField};
use nphysics::math::Isometry;
use nphysics::object::{DefaultColliderHandle, DefaultColliderSet};
use simba::scalar::SubsetOf;

pub struct Capsule {
    color: Point3<f32>,
    base_color: Point3<f32>,
    delta: Isometry<f32>,
    gfx: GraphicsNode,
    collider: DefaultColliderHandle,
}

impl Capsule {
    pub fn new<N: RealField + SubsetOf<f32>>(
        collider: DefaultColliderHandle,
        colliders: &DefaultColliderSet<N>,
        delta: Isometry<f32>,
        r: f32,
        h: f32,
        color: Point3<f32>,
        window: &mut window::Window,
    ) -> Capsule {
        #[cfg(feature = "dim2")]
        let node = window.add_planar_capsule(r, h);
        #[cfg(feature = "dim3")]
        let node = window.add_capsule(r, h);

        let mut res = Capsule {
            color,
            base_color: color,
            delta,
            gfx: node,
            collider,
        };

        let pos: Isometry<f32> = na::convert(*colliders.get(collider).unwrap().position());

        if colliders
            .get(collider)
            .unwrap()
            .query_type()
            .is_proximity_query()
        {
            res.gfx.set_surface_rendering_activation(false);
            res.gfx.set_lines_width(1.0);
        }
        res.gfx.set_color(color.x, color.y, color.z);
        res.gfx.set_local_transformation(pos * res.delta);
        res.update(colliders);

        res
    }

    pub fn select(&mut self) {
        self.color = Point3::new(1.0, 0.0, 0.0);
    }

    pub fn unselect(&mut self) {
        self.color = self.base_color;
    }

    pub fn update<N: RealField + SubsetOf<f32>>(&mut self, colliders: &DefaultColliderSet<N>) {
        node::update_scene_node(
            &mut self.gfx,
            colliders,
            self.collider,
            &self.color,
            &self.delta,
        );
    }

    pub fn set_color(&mut self, color: Point3<f32>) {
        self.gfx.set_color(color.x, color.y, color.z);
        self.color = color;
        self.base_color = color;
    }

    pub fn scene_node(&self) -> &GraphicsNode {
        &self.gfx
    }

    pub fn scene_node_mut(&mut self) -> &mut GraphicsNode {
        &mut self.gfx
    }

    pub fn object(&self) -> DefaultColliderHandle {
        self.collider
    }
}
