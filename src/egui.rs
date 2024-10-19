use std::f64::consts::PI;

use egui::{Align2, Color32, Context, Frame, Id, Pos2, Rect, Sense, Vec2};
use nalgebra::{Matrix4, Point3, Vector3 as NVec3};
use transform_gizmo_egui::{
    enum_set,
    math::Transform,
    mint::{Quaternion, RowMatrix4, Vector3, Vector4},
    Gizmo, GizmoConfig, GizmoExt, GizmoMode, GizmoVisuals,
};

use crate::egui_render::AppState;

const gizmo_legth_side: f32 = 220.0;

/// This is the function that the egui renderer renders. This is what's most applicable in a cross application format
pub fn gui(ui: &Context, app_state: &mut AppState) {
    let mut window_pos = Pos2::ZERO;
    let mut window_size = Vec2::ZERO;
    egui::Window::new("Captcha Controls")
        // .vscroll(true)
        .default_open(true)
        .default_width(800.0)
        .default_height(1000.0)
        .resizable(false)
        .movable(true)
        .show(ui, |ui| {
            ui.label("Controls:");
            ui.label("\tScroll Wheel: Zoom In and Out");
            ui.label("\tLeft Mouse Button Click: Place Block");
            ui.label("\tRight Mouse Button Click: Remove Block");
            ui.label("\tUse Gimbal for Rotation");

            // Store the window's position and size
            window_pos = ui.min_rect().min;
            window_size = ui.min_rect().size();

            // Calculate gizmo size and position
            let gizmo_size = Vec2::new(gizmo_legth_side, gizmo_legth_side);
            let gizmo_pos = window_pos + Vec2::new(10.0, 90.0); // Adjust these offsets as needed

            let mut transform = Transform::from_scale_rotation_translation(
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                app_state.rotation,
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            );

            // Create a non-interactable sub-area for the gizmo
            // Create a non-draggable area for the gizmo
            let gizmo_rect = Rect::from_min_size(gizmo_pos, gizmo_size);

            let gizmo_response = ui.allocate_rect(gizmo_rect, Sense::click_and_drag());

            if gizmo_response.clicked() || gizmo_response.dragged() {}

            let gizmo_rect = gizmo_response.rect;

            // Draw the background and border
            ui.painter()
                .rect_filled(gizmo_rect, 0.0, Color32::from_black_alpha(10));
            ui.painter()
                .rect_stroke(gizmo_rect, 0.0, (1.0, Color32::WHITE));

            let (view_matrix, projection_matrix) = make_matrices();

            let gizmo_config = GizmoConfig {
                view_matrix,
                projection_matrix,
                viewport: gizmo_rect,
                modes: enum_set!(GizmoMode::RotateX | GizmoMode::RotateY | GizmoMode::RotateZ),
                ..Default::default()
            };
            app_state.gizmo.update_config(gizmo_config);

            if let Some((result, new_transforms)) = app_state.gizmo.interact(&ui, &[transform]) {
                for (new_transform, transform) in
                    new_transforms.iter().zip(std::iter::once(&mut transform))
                {
                    *transform = *new_transform;
                }
            }
            if app_state.rotation != transform.rotation {
                app_state.rotation = transform.rotation;
            }
        });
}

// This recalculates every frame. Fix it later
fn make_matrices() -> (RowMatrix4<f64>, RowMatrix4<f64>) {
    // Define the camera position, looking down the diagonal at 45-degree angles
    let aspect_ratio = 1.;
    let fov = std::f64::consts::FRAC_PI_4;
    // Clipping planes. Cannot be the same and near must be above 0
    let near = 0.1;
    let far = 1.;
    let mut eye = Point3::new(1.0, 1.0, 1.0);
    let eye_vec = NVec3::new(eye.x, eye.y, eye.z).normalize(); // Normalize the vector
    eye = Point3::new(eye_vec.x, eye_vec.y, eye_vec.z); // Convert back to Point3
    let target = Point3::new(0.0, 0.0, 0.0); // Looking at the origin
    let up = NVec3::new(0.0, 1.0, 0.0); // World "up" direction

    // Create the view matrix using nalgebra's look_at function
    let view_matrix: Matrix4<f64> = Matrix4::look_at_rh(&eye, &target, &up);

    // Create the perspective projection matrix using field of view and aspect ratio
    let projection_matrix: Matrix4<f64> = Matrix4::new_perspective(aspect_ratio, fov, near, far);

    // Convert the view matrix to RowMatrix4
    let view_matrix = RowMatrix4 {
        x: Vector4 {
            x: view_matrix[(0, 0)],
            y: view_matrix[(0, 1)],
            z: view_matrix[(0, 2)],
            w: view_matrix[(0, 3)],
        },
        y: Vector4 {
            x: view_matrix[(1, 0)],
            y: view_matrix[(1, 1)],
            z: view_matrix[(1, 2)],
            w: view_matrix[(1, 3)],
        },
        z: Vector4 {
            x: view_matrix[(2, 0)],
            y: view_matrix[(2, 1)],
            z: view_matrix[(2, 2)],
            w: view_matrix[(2, 3)],
        },
        w: Vector4 {
            x: view_matrix[(3, 0)],
            y: view_matrix[(3, 1)],
            z: view_matrix[(3, 2)],
            w: view_matrix[(3, 3)],
        },
    };

    // Convert the projection matrix to RowMatrix4
    let projection_matrix = RowMatrix4 {
        x: Vector4 {
            x: projection_matrix[(0, 0)],
            y: projection_matrix[(0, 1)],
            z: projection_matrix[(0, 2)],
            w: projection_matrix[(0, 3)],
        },
        y: Vector4 {
            x: projection_matrix[(1, 0)],
            y: projection_matrix[(1, 1)],
            z: projection_matrix[(1, 2)],
            w: projection_matrix[(1, 3)],
        },
        z: Vector4 {
            x: projection_matrix[(2, 0)],
            y: projection_matrix[(2, 1)],
            z: projection_matrix[(2, 2)],
            w: projection_matrix[(2, 3)],
        },
        w: Vector4 {
            x: projection_matrix[(3, 0)],
            y: projection_matrix[(3, 1)],
            z: projection_matrix[(3, 2)],
            w: projection_matrix[(3, 3)],
        },
    };

    (view_matrix, projection_matrix)
}
