use crate::app::StorageCleaner;
use gpui::prelude::*;
use gpui::*;

pub fn render_app(
    app: &mut StorageCleaner,
    _window: &mut Window,
    cx: &mut Context<StorageCleaner>,
) -> impl IntoElement {
    let selected_count = app.selected_count();
    let total_size_gb = app.total_selected_size_gb();

    div()
        .size_full()
        .flex()
        .flex_col()
        .bg(rgb(0x1e1e1e))
        .text_color(rgb(0xcccccc))
        .child(render_header(app, selected_count, total_size_gb, cx))
        .child(render_project_list(app, cx))
}

fn render_header(
    app: &StorageCleaner,
    selected_count: usize,
    total_size_gb: f64,
    cx: &mut Context<StorageCleaner>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .p_4()
        .gap_3()
        .bg(rgb(0x252526))
        .border_b_1()
        .border_color(rgb(0x3e3e42))
        .child(
            div()
                .text_2xl()
                .font_weight(FontWeight::BOLD)
                .child("Dev Storage Cleaner"),
        )
        .child(render_controls(app, selected_count, cx))
        .child(
            div()
                .text_sm()
                .text_color(rgb(0x858585))
                .child(format!("Scanning: {}", app.config.scan_path.display())),
        )
        .child(
            div()
                .id("change_dir_button")
                .cursor_pointer()
                .px_3()
                .py_1()
                .text_sm()
                .bg(rgb(0x3c3c3c))
                .rounded_md()
                .border_1()
                .border_color(rgb(0x5a5a5a))
                .child("📁 Change Directory...")
                .hover(|style| style.bg(rgb(0x4a4a4a)))
                .on_click(cx.listener(|view, _event, _window, cx| {
                    // Open directory picker
                    if let Some(path) = native_dialog::FileDialog::new()
                        .set_location(&view.config.scan_path)
                        .show_open_single_dir()
                        .ok()
                        .flatten()
                    {
                        view.set_scan_path(path);
                        cx.notify();
                    }
                })),
        )
        .child(div().text_sm().child(app.status_message.clone()))
        .when(selected_count > 0, |this| {
            this.child(
                div()
                    .text_sm()
                    .text_color(rgb(0x4ec9b0))
                    .child(format!("Selected: {:.2} GB", total_size_gb)),
            )
        })
}

fn render_controls(
    app: &StorageCleaner,
    selected_count: usize,
    cx: &mut Context<StorageCleaner>,
) -> impl IntoElement {
    div()
        .flex()
        .gap_3()
        .items_center()
        .child(div().child("Threshold (days):"))
        .child(
            div()
                .px_2()
                .py_1()
                .bg(rgb(0x3c3c3c))
                .rounded_md()
                .child(app.config.threshold_days.to_string()),
        )
        .child(
            div()
                .id("scan_button")
                .cursor_pointer()
                .px_4()
                .py_2()
                .bg(rgb(0x0e639c))
                .rounded_md()
                .border_1()
                .border_color(rgb(0x007acc))
                .child(if app.is_scanning {
                    "Scanning..."
                } else {
                    "Scan"
                })
                .hover(|style| style.bg(rgb(0x1177bb)))
                .on_click(cx.listener(|view, _event, _window, cx| {
                    if !view.is_scanning {
                        view.scan_for_projects();
                        cx.notify();
                    }
                })),
        )
        .child(
            div()
                .id("delete_button")
                .cursor_pointer()
                .px_4()
                .py_2()
                .bg(if selected_count > 0 {
                    rgb(0xc72e0f)
                } else {
                    rgb(0x3c3c3c)
                })
                .rounded_md()
                .border_1()
                .border_color(if selected_count > 0 {
                    rgb(0xf14c4c)
                } else {
                    rgb(0x5a5a5a)
                })
                .child(format!("Delete Selected ({})", selected_count))
                .when(selected_count > 0, |div| {
                    div.hover(|style| style.bg(rgb(0xe03e1b)))
                })
                .on_click(cx.listener(|view, _event, _window, cx| {
                    if view.selected_count() > 0 {
                        view.delete_selected();
                        cx.notify();
                    }
                })),
        )
}

fn render_project_list(app: &StorageCleaner, cx: &mut Context<StorageCleaner>) -> impl IntoElement {
    div()
        .id("project_list")
        .flex()
        .flex_col()
        .flex_1()
        .overflow_y_scroll()
        .p_4()
        .gap_2()
        .children(
            app.projects
                .iter()
                .enumerate()
                .map(|(index, project)| render_project_card(project, index, cx)),
        )
        .when(app.projects.is_empty() && !app.is_scanning, |this| {
            this.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_1()
                    .text_color(rgb(0x858585))
                    .child("No old projects found. Click 'Scan' to search."),
            )
        })
}

fn render_project_card(
    project: &crate::scanner::ProjectInfo,
    index: usize,
    cx: &mut Context<StorageCleaner>,
) -> impl IntoElement {
    let selected = project.selected;
    div()
        .id(("project", index))
        .flex()
        .p_3()
        .gap_3()
        .bg(if selected {
            rgb(0x37373d)
        } else {
            rgb(0x2d2d30)
        })
        .rounded_md()
        .border_1()
        .border_color(if selected {
            rgb(0x007acc)
        } else {
            rgb(0x3e3e42)
        })
        .cursor_pointer()
        .hover(|style| {
            style.bg(if selected {
                rgb(0x3e3e46)
            } else {
                rgb(0x333337)
            })
        })
        .on_click(cx.listener(move |view, _event, _window, cx| {
            view.toggle_project(index);
            cx.notify();
        }))
        .child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .size_5()
                .border_1()
                .border_color(rgb(0x6e6e6e))
                .rounded_sm()
                .when(selected, |this| {
                    this.bg(rgb(0x007acc))
                        .child(div().text_color(rgb(0xffffff)).child("✓"))
                }),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .flex_1()
                .child(
                    div()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xcccccc))
                        .child(
                            project
                                .project_path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unknown")
                                .to_string(),
                        ),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x858585))
                        .child(format!("{}", project.project_path.display())),
                )
                .child(
                    div()
                        .flex()
                        .gap_3()
                        .text_xs()
                        .text_color(rgb(0x858585))
                        .child(format!("{} days old", project.days_old()))
                        .child("•")
                        .child(format!("{:.2} GB", project.size_gb())),
                ),
        )
}
