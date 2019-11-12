use legion::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos(f32, f32, f32);
#[derive(Clone, Copy, Debug, PartialEq)]
struct Vel(f32, f32, f32);

fn main() {
    // create world
    let universe = Universe::new();
    let mut world = universe.create_world();

    // create entities
    world.insert(
        (),
        vec![
            (Pos(1., 2., 3.), Vel(1., 2., 3.)),
            (Pos(1., 2., 3.), Vel(1., 2., 3.)),
            (Pos(1., 2., 3.), Vel(1., 2., 3.)),
            (Pos(1., 2., 3.), Vel(1., 2., 3.)),
        ],
    );

    // update positions
    let mut query = <(Write<Pos>, Read<Vel>)>::query();
    for (mut pos, vel) in query.iter(&mut world) {
        pos.0 += vel.0;
        pos.1 += vel.1;
        pos.2 += vel.2;
    }

    #[cfg(feature = "par-iter")]
    {
        // update positions in parallel
        let mut query = <(Write<Pos>, Read<Vel>)>::query();
        query.par_for_each(&mut world, |(mut pos, vel)| {
            pos.0 += vel.0;
            pos.1 += vel.1;
            pos.2 += vel.2;
        });
    }
}
