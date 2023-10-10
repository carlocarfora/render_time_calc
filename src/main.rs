use eframe::egui;

fn calculate_render_time(mins:i32, frames:i32, machines:i32) -> String{
    if machines == 0 {
        let total = "0 hours";
        total.to_string()
    } else {
        let total_mins:f32 = ((mins * frames) / machines) as f32;

        let hours:f32 = (total_mins / 60.0).floor();
        let minutes:f32 = total_mins % 60.0;
        
        let total = format!("{:.0} h {:.0} m", hours, minutes);
        total
    }
}

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(420.0, 240.0)),
        resizable: false,
        ..Default::default()
    };

    // Our application state:
    let mut minutes = 0;
    let mut frames = 0;
    let mut machines = 0;

    // showing the UI
    eframe::run_simple_native("Render Time Calc", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();
            ui.label("Enter how long a frame takes to render, how long it takes and how many machines you will have to get a render time estimate.");
            ui.separator();

            ui.heading("Estimated Render Time");
            ui.add_space(10.0);
            egui::Grid::new("some_unique_id")
                .min_col_width(100.0)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Minutes per Frame: ");
                    ui.add(egui::Slider::new(&mut minutes, 0..=120));
                    ui.end_row();
                
                    ui.label("No. of Frames: ");
                    ui.add(egui::Slider::new(&mut frames, 0..=100));
                    ui.end_row();
                
                    ui.label("No. of Machines: ");
                    ui.add(egui::Slider::new(&mut machines, 0..=100));
                    ui.end_row();
                });

                ui.add_space(10.0);
                ui.heading(format!("Render Time: {}", calculate_render_time(minutes, frames, machines)));
        });
 
    })
}