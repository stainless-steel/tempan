#![allow(non_snake_case)]

use assert;
use matrix::format::{Conventional, Diagonal};
use std::path::PathBuf;
use temperature::circuit::ThreeDICE;
use temperature::{Circuit, Config, Simulator};

mod fixture;

const UNITS: usize = 4;

#[test]
fn ambience() {
    let mut simulator = setup("004.stk");
    let mut Q = vec![0.0; 42 * UNITS];
    simulator.step(&vec![0.0; 42 * UNITS], &mut Q);
    assert::close(&Q, &vec![318.15; 42 * UNITS], 0.0);
}

#[test]
fn conductance() {
    use matrix::{Size, Matrix};
    use matrix::decomposition::SymmetricEigen;
    use matrix::operation::{MultiplyInto, MultiplySelf, Transpose};

    let Circuit { conductance, distribution, .. } = ThreeDICE::new(find("004.stk")).unwrap();
    let (nodes, units) = distribution.dimensions();

    let mut A = Conventional::zero(nodes);
    {
        let mut U = Conventional::from(conductance);
        let mut L = Diagonal::zero(nodes);
        SymmetricEigen::decompose(&mut (&mut *U, &mut *L)).unwrap();
        for lambda in L.iter_mut() {
            *lambda = 1.0 / *lambda;
        }
        U.multiply_self(&L);
        U.multiply_into(&U.transpose(), &mut A);
    }

    let mut P = vec![];
    for i in 0..units {
        P.push(10.0 * (i as f64 + 1.0));
    }
    P.extend(vec![0.0; nodes - units]);

    let mut Q = vec![318.15; nodes];
    A.multiply_into(&P, &mut Q);
    Q.truncate(units);

    assert::close(&Q, &vec![3.556272578548002e+02, 3.856526078345531e+02,
                            4.156779578143060e+02, 4.457033077940591e+02], 1e-14);
}

#[test]
fn distribution() {
    let Circuit { distribution, .. } = ThreeDICE::new(find("004.stk")).unwrap();
    let distribution = Conventional::from(&distribution);
    let identity = Conventional::from(Diagonal::from_vec((4 * UNITS, UNITS), vec![1.0; UNITS]));
    assert::close(&*distribution, &*identity, 1e-15);
}

#[test]
fn step_20() {
    let mut simulator = setup("004.stk");
    let mut Q = vec![0.0; 20 * UNITS];
    simulator.step(&fixture::P, &mut Q);
    assert::close(&Q, &fixture::Q[..], 1e-12);
}

fn setup(name: &str) -> Simulator {
    let circuit = ThreeDICE::new(find(name)).unwrap();
    Simulator::new(&circuit, &Config::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/threed_ice/fixtures").join(name);
    assert!(::std::fs::metadata(&path).is_ok());
    path
}