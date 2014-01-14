//! Trait implemented by every integrators.

use ncollide::math::N;

/// Trait implemented by every integrator.
///
/// An integrator is a structure capable of updating a dynamic body position and orientation after a given time-step.
pub trait Integrator<O> {
    /// Updates the position and orientation of the object `o` after a time step of `dt`.
    fn update(&mut self, dt: N, o: &mut O);
}
