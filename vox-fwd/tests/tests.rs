use nalgebra::{Point3, Translation3};

use vox_fwd::{Transform3, coordinate_systems as cs, coordinate_systems::transforms::{Tx, WorldFromModel}, safety::{TaggedAs, Taggable, tag}, Pt3};
use vox_fwd::coordinate_systems::InModel;

#[test]
fn test_cs_transformations() {
    let b : cs::PtWorld = <WorldFromModel>::identity() * cs::PtModel::new([1.0, 2.0, 3.0]);

    assert_eq!(b.value.coords.as_slice(), &[1.0, 2.0, 3.0]);

    let tx = Translation3::from([1.0, 1.0, 1.0]).into_tagged::<cs::In<cs::World>>();

    let c = tx * b;

    // let g = Translation3::from([1.0, 1.0, 1.0]) * Point3::from([1.0, 1.0, 1.0]);

    assert_eq!(c.value.coords.as_slice(), &[2.0, 3.0, 4.0]);

    let txa = InModel::from(Translation3::<f32>::identity());
    let tx = tag::<Tx<cs::View, cs::World>, _>(Translation3::from([1.0, 1.0, 1.0]));
}
