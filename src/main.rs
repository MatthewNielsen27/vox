use std::path::Path;
use std::sync::Arc;

use nalgebra as na;

use vox::grr;
use vox::camera::CameraInfo;
use vox::model::Mesh;
use vox::surface::Surface;

fn main() {
    let mut surface = Surface::new(1920, 1080);

    let camera = CameraInfo {
        eye:    na::Point3::new(0.0, 0.0, 3.0),
        target: na::Point3::new(0.0, 0.0, 0.0)
    };

    let proj  = nalgebra::Perspective3::new(1920.0 / 1080.0, 3.14 / 2.0, 1.0, 100.0);

    // Let's create a model in the given scene.
    let model_path = Path::new("resources/sphere-ascii.stl");
    let model = vox::model::Model {
        mesh: Arc::new(Mesh::from_facets(vox_stl::stl::parse_from_file(model_path).unwrap())),
        transform: na::convert(na::Isometry3::translation(0.0, 0.0, 0.0))
    };
    grr::render_model(&model, &camera, &proj, &mut surface, &[255, 255, 255]);

    let model_path = Path::new("resources/cube_ascii.stl");
    let mut model = vox::model::Model {
        mesh: Arc::new(Mesh::from_facets(vox_stl::stl::parse_from_file(model_path).unwrap())),
        transform: na::convert(na::Isometry3::translation(0.5, 0.5, 0.0))
    };
    grr::render_model(&model, &camera, &proj, &mut surface, &[0, 0, 255]);

    model.transform = na::convert(na::Isometry3::translation(-1.5, 0.5, -3.0));
    grr::render_model(&model, &camera, &proj, &mut surface, &[0, 255, 0]);

    surface.to_img().save("scene.png").unwrap();
}
