use eframe::egui;
use crate::models::*;
use crate::factories::*;
use crate::strategy::*;
use crate::observer::*;
use crate::composite::*;
use chrono::{Local, Duration};
use std::sync::Arc;
use parking_lot::RwLock;
use uuid::Uuid;

#[derive(Default)]
pub struct BookingApp {
    rooms: Arc<RwLock<Vec<Room>>>,
    bookings: Arc<RwLock<Vec<Booking>>>,
    users: Arc<RwLock<Vec<User>>>,
    current_user: Option<User>,
    notification_system: Arc<NotificationSystem>,
    
    // UI State
    selected_room: Option<usize>,
    selected_tab: Tab,
    
    // New booking form
    new_booking_purpose: String,
    new_booking_attendees: String,
    new_booking_duration: f32,
    
    // New room form
    new_room_name: String,
    new_room_type: RoomType,
    new_room_capacity: String,
    new_room_floor: String,
    
    // User form
    new_user_name: String,
    new_user_email: String,
    new_user_department: String,
    
    status_message: String,
}

#[derive(PartialEq)]
enum Tab {
    Dashboard,
    Bookings,
    Rooms,
    Users,
    Settings,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Dashboard
    }
}

impl BookingApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        app.initialize();
        
        // Setup custom fonts and style
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals.window_rounding = 8.0.into();
        style.visuals.window_shadow.extrusion = 16.0;
        cc.egui_ctx.set_style(style);
        
        app
    }

    fn initialize(&mut self) {
        // Initialize notification system
        self.notification_system = Arc::new(NotificationSystem::new());
        self.notification_system.subscribe(Arc::new(EmailNotifier::new()));
        self.notification_system.subscribe(Arc::new(SMSNotifier::new()));
        self.notification_system.subscribe(Arc::new(AuditLogger::new()));

        // Create sample rooms using Factory Pattern
        let mut rooms = vec![
            SimpleRoomFactory::create_room(RoomType::ConferenceRoom, "Executive Board Room".to_string(), 5),
            SimpleRoomFactory::create_room(RoomType::MeetingRoom, "Team Room A".to_string(), 2),
            SimpleRoomFactory::create_room(RoomType::MeetingRoom, "Team Room B".to_string(), 2),
            SimpleRoomFactory::create_room(RoomType::TrainingRoom, "Training Center".to_string(), 3),
            SimpleRoomFactory::create_room(RoomType::ExecutiveSuite, "CEO Suite".to_string(), 10),
            SimpleRoomFactory::create_room(RoomType::Auditorium, "Main Auditorium".to_string(), 1),
        ];
        
        *self.rooms.write() = rooms;

        // Create sample users
        let users = vec![
            User::new("John Doe".to_string(), "john@company.com".to_string(), "Engineering".to_string()),
            User::new("Jane Smith".to_string(), "jane@company.com".to_string(), "Marketing".to_string())
                .with_role(UserRole::Manager),
            User::new("Admin User".to_string(), "admin@company.com".to_string(), "IT".to_string())
                .with_role(UserRole::Administrator),
        ];
        
        *self.users.write() = users.clone();
        self.current_user = Some(users[0].clone());

        // Create sample bookings
        let room_id = self.rooms.read()[0].id;
        let user_id = users[0].id;
        
        let booking = Booking::new(
            room_id,
            user_id,
            Local::now() + Duration::hours(2),
            Local::now() + Duration::hours(4),
            "Weekly Team Standup".to_string(),
            8,
        );
        
        self.bookings.write().push(booking);
    }

    fn draw_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading("📊 Dashboard");
        ui.add_space(10.0);

        let rooms_count = self.rooms.read().len();
        let bookings_count = self.bookings.read().len();
        let users_count = self.users.read().len();

        ui.horizontal(|ui| {
            self.stat_card(ui, "Total Rooms", rooms_count, egui::Color32::from_rgb(52, 152, 219));
            self.stat_card(ui, "Active Bookings", bookings_count, egui::Color32::from_rgb(46, 204, 113));
            self.stat_card(ui, "Total Users", users_count, egui::Color32::from_rgb(155, 89, 182));
        });

        ui.add_space(20.0);
        ui.heading("Recent Bookings");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for booking in self.bookings.read().iter().take(5) {
                self.draw_booking_card(ui, booking);
            }
        });
    }

    fn stat_card(&self, ui: &mut egui::Ui, label: &str, value: usize, color: egui::Color32) {
        egui::Frame::none()
            .fill(color)
            .rounding(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(value.to_string())
                        .size(32.0)
                        .color(egui::Color32::WHITE)
                        .strong());
                    ui.label(egui::RichText::new(label)
                        .size(14.0)
                        .color(egui::Color32::WHITE));
                });
            });
    }

    fn draw_booking_card(&self, ui: &mut egui::Ui, booking: &Booking) {
        let room_name = self.rooms.read()
            .iter()
            .find(|r| r.id == booking.room_id)
            .map(|r| r.name.clone())
            .unwrap_or_else(|| "Unknown Room".to_string());

        egui::Frame::none()
            .fill(egui::Color32::from_gray(240))
            .rounding(6.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&room_name).strong());
                    ui.label("-");
                    ui.label(&booking.purpose);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("Status: {}", booking.status));
                    });
                });
                ui.label(format!("🕐 {} - {}", 
                    booking.start_time.format("%Y-%m-%d %H:%M"),
                    booking.end_time.format("%H:%M")));
            });
        ui.add_space(8.0);
    }

    fn draw_bookings(&mut self, ui: &mut egui::Ui) {
        ui.heading("📅 Bookings Management");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("➕ New Booking").clicked() {
                // Open new booking form
            }
        });

        ui.separator();

        egui::Grid::new("bookings_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Room").strong());
                ui.label(egui::RichText::new("Purpose").strong());
                ui.label(egui::RichText::new("Time").strong());
                ui.label(egui::RichText::new("Status").strong());
                ui.label(egui::RichText::new("Actions").strong());
                ui.end_row();

                let bookings = self.bookings.read().clone();
                for (idx, booking) in bookings.iter().enumerate() {
                    let room_name = self.rooms.read()
                        .iter()
                        .find(|r| r.id == booking.room_id)
                        .map(|r| r.name.clone())
                        .unwrap_or_else(|| "Unknown".to_string());

                    ui.label(&room_name);
                    ui.label(&booking.purpose);
                    ui.label(booking.start_time.format("%Y-%m-%d %H:%M").to_string());
                    ui.label(booking.status.to_string());
                    
                    ui.horizontal(|ui| {
                        if booking.status == BookingStatus::Pending && ui.button("✓ Confirm").clicked() {
                            let mut bookings = self.bookings.write();
                            bookings[idx].confirm();
                            self.notification_system.notify_confirmed(&bookings[idx]);
                        }
                        if booking.status != BookingStatus::Cancelled && ui.button("✗ Cancel").clicked() {
                            let mut bookings = self.bookings.write();
                            bookings[idx].cancel();
                            self.notification_system.notify_cancelled(&bookings[idx]);
                        }
                    });
                    ui.end_row();
                }
            });
    }

    fn draw_rooms(&mut self, ui: &mut egui::Ui) {
        ui.heading("🏢 Rooms Management");
        ui.add_space(10.0);

        ui.collapsing("➕ Add New Room", |ui| {
            self.draw_new_room_form(ui);
        });

        ui.separator();

        egui::Grid::new("rooms_grid")
            .striped(true)
            .min_col_width(80.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Name").strong());
                ui.label(egui::RichText::new("Type").strong());
                ui.label(egui::RichText::new("Capacity").strong());
                ui.label(egui::RichText::new("Floor").strong());
                ui.label(egui::RichText::new("Rate/hr").strong());
                ui.label(egui::RichText::new("Amenities").strong());
                ui.end_row();

                for room in self.rooms.read().iter() {
                    ui.label(&room.name);
                    ui.label(room.room_type.to_string());
                    ui.label(room.capacity.to_string());
                    ui.label(room.floor.to_string());
                    ui.label(format!("${:.2}", room.hourly_rate));
                    
                    let mut amenities = Vec::new();
                    if room.has_projector { amenities.push("📽️"); }
                    if room.has_whiteboard { amenities.push("📋"); }
                    if room.has_video_conf { amenities.push("📹"); }
                    ui.label(amenities.join(" "));
                    
                    ui.end_row();
                }
            });
    }

    fn draw_new_room_form(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.new_room_name);
        });

        ui.horizontal(|ui| {
            ui.label("Type:");
            egui::ComboBox::from_label("")
                .selected_text(self.new_room_type.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.new_room_type, RoomType::MeetingRoom, "Meeting Room");
                    ui.selectable_value(&mut self.new_room_type, RoomType::ConferenceRoom, "Conference Room");
                    ui.selectable_value(&mut self.new_room_type, RoomType::TrainingRoom, "Training Room");
                    ui.selectable_value(&mut self.new_room_type, RoomType::ExecutiveSuite, "Executive Suite");
                    ui.selectable_value(&mut self.new_room_type, RoomType::Auditorium, "Auditorium");
                });
        });

        ui.horizontal(|ui| {
            ui.label("Capacity:");
            ui.text_edit_singleline(&mut self.new_room_capacity);
        });

        ui.horizontal(|ui| {
            ui.label("Floor:");
            ui.text_edit_singleline(&mut self.new_room_floor);
        });

        if ui.button("Create Room").clicked() {
            if let (Ok(capacity), Ok(floor)) = (
                self.new_room_capacity.parse::<u32>(),
                self.new_room_floor.parse::<u32>()
            ) {
                let room = SimpleRoomFactory::create_room(
                    self.new_room_type.clone(),
                    self.new_room_name.clone(),
                    floor
                );
                self.rooms.write().push(room);
                self.status_message = "Room created successfully!".to_string();
                
                // Clear form
                self.new_room_name.clear();
                self.new_room_capacity.clear();
                self.new_room_floor.clear();
            } else {
                self.status_message = "Invalid capacity or floor number!".to_string();
            }
        }
    }

    fn draw_users(&mut self, ui: &mut egui::Ui) {
        ui.heading("👥 Users Management");
        ui.add_space(10.0);

        egui::Grid::new("users_grid")
            .striped(true)
            .min_col_width(120.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Name").strong());
                ui.label(egui::RichText::new("Email").strong());
                ui.label(egui::RichText::new("Department").strong());
                ui.label(egui::RichText::new("Role").strong());
                ui.end_row();

                for user in self.users.read().iter() {
                    ui.label(&user.name);
                    ui.label(&user.email);
                    ui.label(&user.department);
                    ui.label(user.role.to_string());
                    ui.end_row();
                }
            });
    }

    fn draw_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙️ Settings");
        ui.add_space(10.0);

        let mut config = crate::config::Config::get();

        ui.horizontal(|ui| {
            ui.label("Max Booking Duration (hours):");
            ui.add(egui::Slider::new(&mut config.max_booking_duration_hours, 1..=24));
        });

        ui.horizontal(|ui| {
            ui.label("Min Booking Duration (hours):");
            ui.add(egui::Slider::new(&mut config.min_booking_duration_hours, 1..=8));
        });

        ui.horizontal(|ui| {
            ui.label("Business Hours Start:");
            ui.add(egui::Slider::new(&mut config.business_hours_start, 0..=23));
        });

        ui.horizontal(|ui| {
            ui.label("Business Hours End:");
            ui.add(egui::Slider::new(&mut config.business_hours_end, 0..=23));
        });

        ui.checkbox(&mut config.allow_concurrent_bookings, "Allow Concurrent Bookings");

        if ui.button("Save Settings").clicked() {
            crate::config::Config::set(config);
            self.status_message = "Settings saved successfully!".to_string();
        }
    }
}

impl eframe::App for BookingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🏢 Room Booking System");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(user) = &self.current_user {
                        ui.label(format!("👤 {}", user.name));
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").min_width(150.0).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                
                if ui.selectable_label(self.selected_tab == Tab::Dashboard, "📊 Dashboard").clicked() {
                    self.selected_tab = Tab::Dashboard;
                }
                if ui.selectable_label(self.selected_tab == Tab::Bookings, "📅 Bookings").clicked() {
                    self.selected_tab = Tab::Bookings;
                }
                if ui.selectable_label(self.selected_tab == Tab::Rooms, "🏢 Rooms").clicked() {
                    self.selected_tab = Tab::Rooms;
                }
                if ui.selectable_label(self.selected_tab == Tab::Users, "👥 Users").clicked() {
                    self.selected_tab = Tab::Users;
                }
                if ui.selectable_label(self.selected_tab == Tab::Settings, "⚙️ Settings").clicked() {
                    self.selected_tab = Tab::Settings;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.selected_tab {
                    Tab::Dashboard => self.draw_dashboard(ui),
                    Tab::Bookings => self.draw_bookings(ui),
                    Tab::Rooms => self.draw_rooms(ui),
                    Tab::Users => self.draw_users(ui),
                    Tab::Settings => self.draw_settings(ui),
                }
                
                if !self.status_message.is_empty() {
                    ui.add_space(10.0);
                    ui.colored_label(egui::Color32::GREEN, &self.status_message);
                }
            });
        });
    }
}
