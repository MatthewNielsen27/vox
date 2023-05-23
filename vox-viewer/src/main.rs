use std::path::Path;
use std::sync::Arc;
use image::Rgb;

use nalgebra as na;
use nalgebra::{Isometry3, Translation3, Vector3};
use nalgebra_glm::Vec3;
use rand::{random, thread_rng};
use rand::distributions::Distribution;

use vox_graphics::{
    texture::MatcapTexture,
    camera::CameraInfo,
    grr,
    surface::Surface,
    model::Model,
    model::Mesh,
};

mod debug_utils;

use crate::debug_utils::random_col;

use minifb::{Key, Window, WindowOptions, ScaleMode, KeyRepeat};

fn move_models(models: &mut Vec<(Model, Rgb<u8>)>){
    let between = rand::distributions::Uniform::from(-1.0..1.0);
    let mut rng = thread_rng();

    for (model, _) in models {
        if random::<f32>() < 0.5 {
            model.transform.append_translation_mut(
                &Translation3::from(
                    Vector3::new(
                        between.sample(&mut rng),
                        between.sample(&mut rng),
                        between.sample(&mut rng)
                    )
                )
            );
        }
    }
}

fn main() {

    let window_w = 600usize;
    let window_h = 400usize;

    let mut buffer: Vec<u32> = vec![0; window_w * window_h];

    let mut window = Window::new(
        "Test - ESC to exit",
        window_w,
        window_h,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_millis(1/60 * 1000)));
    window.limit_update_rate(Some(std::time::Duration::from_millis(((1.0/30.0) * 1000.0) as u64 )));

    let mut surface = Surface::new(window_w, window_h, true);

    let mut camera = CameraInfo::new(
        na::Point3::new(0.0, 0.0, -5.0),
        na::Point3::new(0.0, 0.0, 0.0),
    );

    let proj  = nalgebra::Perspective3::new(window_w as f32 / window_h as f32, /*3.14 / 2.0*/ 3.14 / 2.0, 1.0, 500.0);

    // Let's create a model in the given scene.
    // let model_path = Path::new("/Users/matthewnielsen/Downloads/Stanford_Bunny.stl");
    // let model_path = Path::new("resources/ascii-sphere.stl");
    // let sphere_mesh = Arc::new(Mesh::from_facets(vox_stl::stl::parse_from_file(model_path).unwrap()));

    let random_xform = || -> na::Similarity3<f32> {
        na::convert(na::Isometry3::translation(random::<f32>() * 10.0,random::<f32>() * 10.0, random::<f32>() * 10.0))
    };

    let matcap1 = Arc::new(MatcapTexture::from_file(&Path::new("resources/matcaps/normal_256.png")));
    // let matcap2 = Arc::new(MatcapTexture::from_file(&Path::new("resources/matcaps/gold_matcap.png")));

    let mesh_path = Path::new("/Users/matthewnielsen/Downloads/3Dbenchy.stl");
    let mesh = Arc::new(
        Mesh::from_facets(
            vox_stl::stl::parse_from_file(mesh_path).unwrap()
        )
    );

    // let sphere = Arc::new(Mesh::from_facets(vox_stl::stl::parse_from_file(Path::new("/Users/matthewnielsen/Downloads/5k_sphere.STL")).unwrap()));

    let mut model1 = Model {
        mesh: mesh.clone(),
        transform: na::convert(Isometry3::translation(0.0,0.0,0.0)),
        texture: matcap1.clone()
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut needsRedraw = false;

        if window.is_key_down(Key::W) {
            needsRedraw = true;
            if window.is_key_down(Key::RightShift) {
                camera.view_matrix.append_translation_mut(&Translation3::from(Vec3::new(0.0,0.0,1.0)));
            } else {
                camera.view_matrix.append_translation_mut(&Translation3::from(Vec3::new(0.0,1.0,0.0)));
            }
        } else if window.is_key_down(Key::S) {
            needsRedraw = true;
            if window.is_key_down(Key::RightShift) {
                camera.view_matrix.append_translation_mut(&Translation3::from(Vec3::new(0.0,0.0,-1.0)));
            } else {
                camera.view_matrix.append_translation_mut(&Translation3::from(Vec3::new(0.0,-1.0,0.0)));
            }
        } else if window.is_key_down(Key::A) {
            needsRedraw = true;
            camera.view_matrix.append_translation_mut(&Translation3::from(Vec3::new(-1.0,0.0,0.0)));
        } else if window.is_key_down(Key::D) {
            needsRedraw = true;
            let v = Translation3::from(Vec3::new(1.0,0.0,0.0));
            camera.view_matrix.append_translation_mut(&v);
        } else if window.is_key_down(Key::Up) {
            needsRedraw = true;
            let rot = na::UnitQuaternion::from_axis_angle(
                &na::Unit::new_normalize(Vector3::x()),
                std::f32::consts::FRAC_PI_4 / 4.0
            );

            camera.view_matrix.append_rotation_wrt_center_mut(&rot);
        } else if window.is_key_down(Key::Down) {
            needsRedraw = true;
            let rot = na::UnitQuaternion::from_axis_angle(
                &na::Unit::new_normalize(Vector3::x()),
                -1.0 * std::f32::consts::FRAC_PI_4 / 4.0
            );
            camera.view_matrix.append_rotation_wrt_center_mut(&rot);
        } else if window.is_key_down(Key::Right) {
            needsRedraw = true;
            let rot = na::UnitQuaternion::from_axis_angle(
                &na::Unit::new_normalize(Vector3::y()),
                std::f32::consts::FRAC_PI_4 / 4.0
            );

            camera.view_matrix.append_rotation_wrt_center_mut(&rot);
        } else if window.is_key_down(Key::Left) {
            needsRedraw = true;
            let rot = na::UnitQuaternion::from_axis_angle(
                &na::Unit::new_normalize(Vector3::y()),
                -1.0 * std::f32::consts::FRAC_PI_4 / 4.0
            );
            camera.view_matrix.append_rotation_wrt_center_mut(&rot);
        } else if window.is_key_down(Key::R) {
            let mut preview_frame = Surface::new(1920, 1080, true);

            let p  = nalgebra::Perspective3::new(
                surface.shape.0 as f32 / surface.shape.1 as f32,
                1.4,
                1.0,
                500.0
            );

            grr::render_model(&model1, &camera, &p, &mut preview_frame);
            preview_frame.to_img().save("/tmp/preview.png").expect("failed to save image");
        }

        if needsRedraw {
            surface.clear();
            grr::render_model(&model1, &camera, &proj, &mut surface);
            // grr::render_model(&model2, &camera, &proj, &mut surface);
            surface.fill_buffer(&mut buffer);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, window_w, window_h)
            .unwrap();
    }
}
