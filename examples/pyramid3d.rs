#[link(name     = "pyramid3d"
       , vers   = "0.0"
       , author = "Sébastien Crozet"
       , uuid   = "9ec127f7-d531-4fbd-b405-9af404cb7aaa")];
#[crate_type = "bin"];
#[warn(non_camel_case_types)]

extern mod std;
extern mod extra;
extern mod nphysics;
extern mod nalgebra;
extern mod ncollide;
extern mod graphics3d;

use nalgebra::mat::Translation;
use nalgebra::vec::Vec3;
use ncollide::geom::{Geom, Box, Plane};
use nphysics::world::BodyWorld;
use nphysics::aliases::dim3;
use nphysics::object::{RigidBody, Static, Dynamic, RB};
use graphics3d::engine::GraphicsManager;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
    GraphicsManager::simulate(pyramid3d)
}

pub fn pyramid3d(graphics: &mut GraphicsManager) -> dim3::BodyWorld3d<f64> {
    /*
     * World
     */
    let mut world = BodyWorld::new();
    world.set_gravity(Vec3::new(0.0f64, -9.81, 0.0));

    /*
     * Planes
     */
    let rb   = RigidBody::new(Geom::new_plane(Plane::new(Vec3::y())), 0.0f64, Static, 0.3, 0.6);
    let body = @mut RB(rb);


    world.add_body(body);
    graphics.add(body);

    /*
     * Create the boxes
     */
    let num     = 30;
    let rad     = 0.5;
    let shift   = rad * 2.0;
    let centerx = shift * (num as f64) / 2.0;
    let centery = shift / 2.0;

    for i in range(0u, num) {
        for j in range(i, num) {
            let fi = i as f64;
            let fj = (j - i) as f64;
            let x = (fi * shift / 2.0) + fj * shift - centerx;
            let y = fi * shift + centery;

            let box    = Box::new(Vec3::new(rad, rad, rad));
            let mut rb = RigidBody::new(Geom::new_box(box), 1.0f64, Dynamic, 0.3, 0.6);

            rb.translate_by(&Vec3::new(x, y, 0.0));

            let body = @mut RB(rb);

            world.add_body(body);
            graphics.add(body);
        }
    }

    /*
     * Set up the camera and that is it!
     */
    graphics.look_at(Vec3::new(0.0, 60.0, -60.0), Vec3::new(0.0, 0.0, 0.0));

    world
}