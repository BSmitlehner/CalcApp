slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_dijeli(|a: f32, b: f32| -> f32 {
        if b != 0.0 { a / b } else { 0.0 }
    });
    ui.on_mnozi(|a: f32, b: f32| -> f32 {
        a * b
    });
    ui.on_zbrajaj(|a: f32, b: f32| -> f32 {
        a + b
    });

    ui.on_oduzmi(|a: f32, b: f32| -> f32 {
        a - b
    });

    ui.run()
}
