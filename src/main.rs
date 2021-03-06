#![allow(unused)]

use crate::charged_particle::ChargedParticle;
use crate::four_momentum::FourMomentum;
use crate::three_vector::ThreeVector;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

mod charged_particle;
mod four_momentum;
mod three_vector;

fn main() {
    let mut particle = ChargedParticle {
        charge: 1.0,
        position: ThreeVector(0.0, 0.0, 0.0),
        momentum: FourMomentum::from_mass_and_velocity(1.0, ThreeVector(0.0, 0.5, 0.0)),
    };
    let mut time: f64 = 0.0;
    let dt: f64 = 0.01;

    /*let field = |_, _| {
        let e = ThreeVector(0.0, 1.0, 0.0);
        let b = ThreeVector(0.0, 0.0, 1.0);
        (e, b)
    };*/

    let e = ThreeVector(0.0, 1.0, 0.0);
    let b = ThreeVector(0.0, 0.0, 1.0);
    let ebyb = e.1 / b.2;

    let save_particle_vec = |solution: &Vec<(f64, ChargedParticle)>| {
        let positionfilename = [
            String::from("../solution/position_"),
            ebyb.to_string(),
            String::from(".dat"),
        ]
        .concat();
        let momentumfilename = [
            String::from("../solution/momentum_"),
            ebyb.to_string(),
            String::from(".dat"),
        ]
        .concat();
        let energyfilename = [
            String::from("../solution/energypowerlaw_"),
            ebyb.to_string(),
            String::from(".dat"),
        ]
        .concat();

        let mut file = File::create(positionfilename).expect("error creating");
        for (t, element) in solution {
            let x = element.position.0;
            let y = element.position.1;
            let z = element.position.2;
            writeln!(&mut file, "{} {} {} {}", t, x, y, z).expect("error writing");
        }

        let mut file = File::create(momentumfilename).expect("error creating");
        for (t, element) in solution {
            let w = element.momentum.0;
            let x = element.momentum.1;
            let y = element.momentum.2;
            let z = element.momentum.3;
            writeln!(&mut file, "{} {} {} {} {}", t, w, x, y, z).expect("error writing");
        }

        let mut file = File::create(energyfilename).expect("error creating");
        for (t, element) in solution {
            let energy = element.total_energy();
            writeln!(&mut file, "{} {}", t, energy).expect("error writing");
        }
    };
    let mut solution = Vec::new();

    while time < 150.0 {
        particle = particle.boris_push(e, b, dt);
        //particle = particle.rk4_push(field, time, dt);
        solution.push((time, particle.clone()));
        time += dt;
    }
    save_particle_vec(&solution);
}
