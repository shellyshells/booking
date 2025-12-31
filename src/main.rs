use room_booking_system::ui::BookingApp;
use room_booking_system::logger::Logger;

fn main() -> Result<(), eframe::Error> {
    // Initialize logger
    Logger::instance().init("booking_system.log");
    Logger::instance().info("Room Booking System starting...");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .unwrap_or_default(),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "Room Booking System",
        options,
        Box::new(|cc| Ok(Box::new(BookingApp::new(cc)))),
    )
}
