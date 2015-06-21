use std::fs;
use std::path::PathBuf;

use assert;
use Analysis;

mod fixture;

#[test]
fn new() {
    let analysis = setup("002");
    let system = analysis.system;

    assert_eq!(system.cores, 2);
    assert_eq!(system.nodes, 4 * 2 + 12);
    assert::close_abs(&system.U, &fixture::U[..], 1e-9);
    assert::close(&system.L, &fixture::L[..], 1e-10);
    assert::close(&system.D, &fixture::D[..], 1e-13);
    assert::close(&system.E, &fixture::E[..], 1e-13);
    assert::close(&system.F, &fixture::F[..], 1e-13);
}

fn setup(name: &str) -> Analysis {
    use std::default::Default;
    use super::model::hotspot::new;

    let circuit = new(&find(&format!("{}.flp", name)), &find("hotspot.config"), "").unwrap();

    Analysis::new(circuit, Default::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests").join("fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}
