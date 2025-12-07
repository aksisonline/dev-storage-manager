use gpui::prelude::*;
use gpui::*;

mod app;
mod config;
mod scanner;
mod ui;

use app::StorageCleaner;

actions!(app_actions, [Scan, Delete, ToggleProject, UpdateThreshold]);

impl Render for StorageCleaner {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ui::render_app(self, window, cx)
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(900.0), px(700.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Dev Storage Cleaner".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| StorageCleaner::new()),
        )
        .unwrap();
    });
}
