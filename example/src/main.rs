use file_log::{index, log};

fn main() {
    log!("log", "Hello"); // this will append "hello" to log_<index>.log
    log!("log" "xyz", "{} {} {}", index(), index(), index()); // this will append "<index> <index> <index>" to log_<index>.xyz

    // Using an atomic simulation as an example

    #[derive(Debug)]
    struct Atom {
        name: String,
        position: (f64, f64, f64),
        velocity: (f64, f64, f64),
    }

    let mut atoms = Vec::with_capacity(1000);
    for x in 0..10 {
        for y in 0..10 {
            for z in 0..10 {
                atoms.push(Atom {
                    name: "H".to_string(),
                    position: (x as f64, y as f64, z as f64),
                    velocity: (0.0, 0.0, 0.0),
                });
            }
        }
    }

    // first, because we are using csv, we need to write the header
    log!("simulation" "csv", "step name x y z vx vy vz");
    // now imagine a simulation loop
    for step in 0..100 {
        for atom in atoms.iter_mut() {
            atom.position.0 += atom.velocity.0;
            atom.position.1 += atom.velocity.1;
            atom.position.2 += atom.velocity.2;
        }
        for atom in atoms.iter() {
            log!("simulation" "csv", "{} {} {} {} {} {} {} {}", step, atom.name, atom.position.0, atom.position.1, atom.position.2, atom.velocity.0, atom.velocity.1, atom.velocity.2);
            // we can also use the Debug trait
            log!("atoms", "{atom:?}");
        }
    }
}
