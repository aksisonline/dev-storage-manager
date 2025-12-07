use crate::app::StorageCleaner;
use gpui::prelude::*;
use gpui::*;

mod theme;
use theme::Theme;

pub fn render_app(
    app: &mut StorageCleaner,
    _window: &mut Window,
    cx: &mut Context<StorageCleaner>,
) -> impl IntoElement {
    let selected_count = app.selected_count();
    let total_size_gb = app.total_selected_size_gb();
    let theme = Theme::zen_dark();

    div()
        .size_full()
        .flex()
        .flex_col()
        .bg(theme.background)
        .text_color(theme.text)
        .child(render_header(app, selected_count, total_size_gb, cx))
        .child(render_project_list(app, cx))
}

fn render_header(
    app: &StorageCleaner,
    selected_count: usize,
    total_size_gb: f64,
    cx: &mut Context<StorageCleaner>,
) -> impl IntoElement {
    let theme = Theme::zen_dark();

    div()
        .flex()
        .flex_col()
        .p_4()
        .gap_3()
        .bg(theme.elevated_surface_background)
        .border_b_1()
        .border_color(theme.border)
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
                .text_color(theme.text_muted)
                .child(format!("Scanning: {}", app.config.scan_path.display())),
        )
        .child(
            div()
                .id("change_dir_button")
                .cursor_pointer()
                .px_3()
                .py_1()
                .text_sm()
                .bg(theme.element_background)
                .rounded_md()
                .border_1()
                .border_color(theme.border_selected)
                .child("📁 Change Directory...")
                .hover(|style| style.bg(theme.element_hover))
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
                    .text_color(theme.text_accent)
                    .child(format!("Selected: {:.2} GB", total_size_gb)),
            )
        })
}

fn render_controls(
    app: &StorageCleaner,
    selected_count: usize,
    cx: &mut Context<StorageCleaner>,
) -> impl IntoElement {
    let theme = Theme::zen_dark();

    div()
        .flex()
        .gap_3()
        .items_center()
        .child(div().child("Threshold (days):"))
        .child(
            div()
                .px_2()
                .py_1()
                .bg(theme.element_background)
                .rounded_md()
                .child(app.config.threshold_days.to_string()),
        )
        .child(
            div()
                .id("scan_button")
                .cursor_pointer()
                .px_4()
                .py_2()
                .bg(theme.info_background)
                .rounded_md()
                .border_1()
                .border_color(theme.info)
                .child(if app.is_scanning {
                    "Scanning..."
                } else {
                    "Scan"
                })
                .hover(|style| style.bg(theme.info))
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
                    theme.error_background
                } else {
                    theme.element_background
                })
                .rounded_md()
                .border_1()
                .border_color(if selected_count > 0 {
                    theme.error
                } else {
                    theme.border_selected
                })
                .child(format!("Delete Selected ({})", selected_count))
                .when(selected_count > 0, |div| {
                    div.hover(|style| style.bg(theme.error))
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
    let theme = Theme::zen_dark();

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
                    .text_color(theme.text_muted)
                    .child("No old projects found. Click 'Scan' to search."),
            )
        })
}

fn render_project_card(
    project: &crate::scanner::ProjectInfo,
    index: usize,
    cx: &mut Context<StorageCleaner>,
) -> impl IntoElement {
    let theme = Theme::zen_dark();
    let selected = project.selected;

    div()
        .id(("project", index))
        .flex()
        .p_3()
        .gap_3()
        .bg(if selected {
            theme.element_selected
        } else {
            theme.editor_background
        })
        .rounded_md()
        .border_1()
        .border_color(if selected {
            theme.border_focused
        } else {
            theme.border
        })
        .cursor_pointer()
        .hover(|style| {
            style.bg(if selected {
                theme.element_active
            } else {
                theme.element_hover
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
                .border_color(theme.border_selected)
                .rounded_sm()
                .when(selected, |this| {
                    this.bg(theme.text_accent)
                        .child(div().text_color(theme.text).child("✓"))
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
                        .text_color(theme.text)
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
                        .text_color(theme.text_muted)
                        .child(format!("{}", project.project_path.display())),
                )
                .child(
                    div()
                        .flex()
                        .gap_3()
                        .text_xs()
                        .text_color(theme.text_placeholder)
                        .child(format!("{} days old", project.days_old()))
                        .child("•")
                        .child(format!("{:.2} GB", project.size_gb())),
                ),
        )
}
