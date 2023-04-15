use cairo::{FontSlant, FontWeight};
use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExtManual},
    traits::GtkWindowExt,
    Application, DrawingArea, Window,
};

fn main() {
    let app = Application::builder()
        .application_id("io.github.aerphanas")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let drawing = DrawingArea::builder().build();

    drawing.set_draw_func(|_, cr, _, _| {
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.paint().unwrap();

        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.select_font_face("Fira Code", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(100.0);
        cr.move_to(50.0, 225.0);
        cr.show_text(".Hello").unwrap();

        cr.set_source_rgb(100.0, 0.0, 100.0);
        cr.select_font_face("Fira Code", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(100.0);
        cr.move_to(100.0, 250.0);
        cr.show_text(".Void").unwrap();
    });

    let window = Window::builder()
        .title("Cairo test")
        .default_width(500)
        .default_height(500)
        .application(app)
        .child(&drawing)
        .build();
    window.present();
}
