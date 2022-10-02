use gamercade_rs::{
    prelude::{self as gc, set_pixel, GraphicsParameters},
};
use rapier2d::{prelude::*, na::Const};
use crate::JUMP_FRAMES;
use std::collections::HashMap;
// Our game state. Edit this as you wish.
pub struct MyGame {
    frame_counter: usize,
    x_pos: i32,
    y_pos: i32,
    jumping: bool,
    falling: bool,
    jump_iter: i32,
    fall_frames: i32,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    body_handles: HashMap<String, RigidBodyHandle>,
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hook: (),
    event_handler: ()
    //jump_height: i32 //this is hard coded into the jump array
}

impl crate::Game for MyGame {
    const FPS: i32 = 60;
    const FPS_USIZE: usize = 60;
    const JUMP_ARRAY: [i32; JUMP_FRAMES] = [0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    const GRAVITY: f32 = 0.5;
    /// Handle all of your initialization logic here.
    fn init() -> Self {
        // We can call Gamercade functions in here.
        gc::console_log("Hello, world!");

        // Initialize our values to 0, and width/height divided
        // by two.
        let mut collider_set = ColliderSet::new();
        let mut rigid_body_set = RigidBodySet::new();
        let mut body_handles = HashMap::new();

        let collider = ColliderBuilder::cuboid(1000.0, 0.1).build();
        collider_set.insert(collider);
        let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![50.0, 100.0])
        .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        body_handles.insert("ball".to_string(),ball_body_handle);

        Self {
            frame_counter: 0,
            x_pos: (gc::width() / 2) as i32,
            y_pos: (gc::height() / 2) as i32,
            jumping: false,
            falling: false,
            fall_frames: 0,
            jump_iter: 0,
            rigid_body_set,
            collider_set,
            body_handles,
            gravity: vector![0.0, -98.1],
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hook: (),
            event_handler: ()
        }
    }

    /// Handle all of your game state logic here
    fn update(&mut self) {
        // Print a message if the user presses the A button.
        // This defaults to the U key on the keyboard.
        if Some(true) == gc::button_a_pressed(0) {
            gc::console_log("Pressed A.");
            self.jumping = true;
        }

        // Let's move the pixel with the arrow keys
        // Handle up/down motion
        if Some(true) == gc::button_up_held(0) {
            self.y_pos -= 1;
        } else if Some(true) == gc::button_down_held(0) {
            self.y_pos += 1;
        }

        // And repeat for left/right
        if Some(true) == gc::button_left_held(0) {
            self.x_pos -= 1;
        } else if Some(true) == gc::button_right_held(0) {
            self.x_pos += 1;
        }

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &self.physics_hook,
            &self.event_handler,
          );
          let items = ["ball"];
          for &item in &items {
            match self.body_handles.get(item) {
                Some(body) => {
                    let ball_body = &mut self.rigid_body_set[*body];
                    gc::console_log(
                        &(format!("Ball altitude: {}",
                        ball_body.translation().y)));
                    if self.jumping{
                        //ball_body.set_translation(vector![50.0, 100.0], true);
                        //assert_eq!(*ball_body.translation(), vector![50.0, 100.0]);
                        ball_body.apply_impulse(vector![0.0, 100.0], true);
                        self.jumping = false;
                    }
                },
                None => ()
            }
          }
    }

    /// Handle all of your rendering code here
    fn draw(&self) {
        // Clear screen function takes a GraphicsParameters as a parameter,
        // so let's make one.
        let clear_color = GraphicsParameters::default().color_index(0);

        // Now we can clear the screen.
        gc::clear_screen(clear_color);

        // Let's draw a pixel.
        let pixel_color = GraphicsParameters::default().color_index(32);
        set_pixel(pixel_color, self.x_pos, self.y_pos);
        let items = ["ball"];
        for &item in &items {
            match self.body_handles.get(item) {
                Some(body) => {
                    let ball_body = &self.rigid_body_set[*body];
                    set_pixel(pixel_color, ball_body.translation().x as i32, ball_body.translation().y as i32);
                },
                None => ()
            }
        }
    }
}
