use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
const UI_WINDOW_WIDTH: u32 = 260;
const UI_WINDOW_HEIGHT: u32 = 140;
const REPEL_RADIUS: f32 = 200.0;

const LINE_LENGTH: f32 = 8.0;

struct Model {
    main_window: WindowId,
    raindrops: Vec<Raindrop>,
    hue: f32,
    min_velocity: f32,
    max_velocity: f32,
    update_velocities: bool,
    ui: Egui,
}

struct Raindrop {
    position: Point2,
    velocity: Vec2,
    color: Hsv,
}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let ui_window = app
        .new_window()
        .size(UI_WINDOW_WIDTH, UI_WINDOW_HEIGHT)
        .view(ui_view)
        .raw_event(raw_ui_event)
        .build()
        .unwrap();

    let ui_window_ref = app.window(ui_window).unwrap();
    let ui = Egui::from_window(&ui_window_ref);

    let horizontal_velocity = random_range(20.0, 40.0);

    let raindrops = (0..1200)
        .map(|_| {
            let x = random_range(WIDTH as f32 * -1.0, WIDTH as f32);
            let y = random_range(300.0, 600.0);
            let position = pt2(x, y);
            let velocity = vec2(horizontal_velocity, random_range(-20.0, -200.0));
            let color = hsv(0.0, 1.0, 1.0);
            Raindrop {
                position,
                velocity,
                color,
            }
        })
        .collect();

    Model {
        main_window,
        raindrops,
        hue: 0.0,
        min_velocity: 20.0,
        max_velocity: 200.0,
        update_velocities: false,
        ui,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let dt = app.duration.since_prev_update.as_secs_f32();
    let main_window_rect = app.window(model.main_window).unwrap();
    let mouse_position = app.mouse.position();

    if model.update_velocities {
        for raindrop in model.raindrops.iter_mut() {
            raindrop.velocity.y = random_range(-model.min_velocity, -model.max_velocity);
        }
        model.update_velocities = false;
    }

    for raindrop in model.raindrops.iter_mut() {
        raindrop.position += raindrop.velocity * dt;

        // Check the distance between the mouse and the raindrop
        let distance = raindrop.position.distance(mouse_position);

        // If the distance is within a certain radius (e.g., 50.0), repel the raindrop
        if distance < REPEL_RADIUS {
            let repel_force =
                (mouse_position - raindrop.position).normalize() * (REPEL_RADIUS - distance) * 2.0;
            raindrop.position -= repel_force * dt;
        }

        if raindrop.position.y < main_window_rect.rect().bottom() {
            raindrop.position.y = main_window_rect.rect().top();
            raindrop.position.x = random_range(WIDTH as f32 * -1.0, WIDTH as f32);
        }
    }

    update_ui(model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for raindrop in &model.raindrops {
        let (start, end) = (
            raindrop.position,
            raindrop.position + vec2(-1.5, LINE_LENGTH),
        );
        draw.line()
            .start(start)
            .end(end)
            .weight(3.0) // Adjust the line thickness here
            .color(raindrop.color);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.add(egui::Slider::new(&mut model.hue, 0.0..=1.0).text("Color Hue"));

            if ui
                .add(egui::Slider::new(&mut model.min_velocity, 1.0..=100.0).text("Min Velocity"))
                .changed()
                || ui
                    .add(
                        egui::Slider::new(&mut model.max_velocity, 100.0..=500.0)
                            .text("Max Velocity"),
                    )
                    .changed()
            {
                model.update_velocities = true;
            }

            for raindrop in &mut model.raindrops {
                raindrop.color = hsv(model.hue, 1.0, 1.0);
            }
        });
}
