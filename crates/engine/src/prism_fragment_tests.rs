//! A blind circular cut must preserve material and wall partitions when the scene moves.

use serde_json::json;
use solidcraft_geom::Vec3;

use crate::Session;

fn first_cut_at(offset: [f64; 3]) -> solidcraft_kernel::Body {
    let mut session = Session::default();
    let plane = if offset == [0.0; 3] { json!("XY") } else { json!({"origin":offset,"x_dir":[1,0,0],"y_dir":[0,1,0]}) };
    for (command, params) in [
        ("sketch.create", json!({"plane":plane})),
        ("sketch.line", json!({"points":[[0,0],[30,0],[30,32.5],[17.5,32.5],[17.5,17.5],[10,17.5],[10,32.5],[0,32.5]]})),
        ("sketch.arc.three_point", json!({"start":[0,32.5],"through":[-15,16.25],"end":[0,0]})),
        ("sketch.finish", json!({})),
        ("solid.extrude", json!({"distance":20})),
        (
            "solid.cylinder",
            json!({"base":[offset[0],offset[1],offset[2]+20.000_000_000_000_01],"axis":[0,0,-1],"radius":6.25,"height":2.5,"operation":"cut"}),
        ),
    ] {
        session.execute(command, &params).unwrap_or_else(|error| panic!("{command}: {error}"));
    }
    assert_eq!(session.model.state().bodies.len(), 1);
    let body = session.model.state().bodies[0].body.clone();
    assert!(body.validity().is_empty());
    body
}

#[test]
fn a_blind_circular_cut_keeps_its_axial_wall_partition_under_translation() {
    let raw = first_cut_at([0.0; 3]);
    let reference = solidcraft_kernel::measure(&raw).unwrap();
    for offset in [[14.999_881_984_669_177, -22.5, -32.500_000_000_000_014], [100.0, -50.0, 25.0]] {
        let shifted = first_cut_at(offset);
        let measured = solidcraft_kernel::measure(&shifted).unwrap();
        assert_eq!(measured.merged, reference.merged);
        for (actual, want) in [(measured.area, reference.area), (measured.volume, reference.volume)] {
            assert!((actual - want).abs() <= 1e-6 * actual.abs().max(want.abs()).max(1.0));
        }
        let displacement = Vec3::new(offset[0], offset[1], offset[2]);
        assert!((measured.centroid - displacement).dist(reference.centroid) < 1e-5);
        // Independent reconstructions of the same arc must not add a slab seam.
        assert_eq!((shifted.face_count(), shifted.edge_count()), (raw.face_count(), raw.edge_count()));
    }
}
