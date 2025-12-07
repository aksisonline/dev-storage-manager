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
    let theme = Theme::coder_black();
    let selected_count = app.selected_count();
    let total_size_gb = app.total_selected_size_gb();

    div()
        .size_full()
        .flex()
        .flex_col()
        .bg(theme.background)
        .text_color(theme.text)
        .font_family("monospace")
        .child(render_header(app, selected_count, total_size_gb, cx))
        .child(render_project_list(app, cx))
}

fn render_header(
    app: &StorageCleaner,
    selected_count: usize,
    total_size_gb: f64,
    cx: &mut Context<StorageCleaner>,
) -> impl IntoElement {
    let theme = Theme::coder_black();

    div()
        .flex()
        .flex_col()
        .p_4()
        .gap_2()
        .bg(theme.surface)
        .border_b_1()
        .border_color(theme.border)
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .child("DEV STORAGE CLEANER"),
                )
                .child(
                    div()
                        .flex()
                        .gap_2()
                        .text_xs()
                        .text_color(theme.text_dim)
                        .child("v1.0"),
                ),
        )
        .child(
            div()
                .flex()
                .gap_2()
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_dim)
                                .child("SCAN PATH"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(format!("{}", app.config.scan_path.display())),
                        ),
                )
                .child(
                    div()
                        .id("change_dir")
                        .cursor_pointer()
                        .px_3()
                        .py_1()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .bg(theme.element_bg)
                        .text_color(theme.text_accent)
                        .border_1()
                        .border_color(theme.border)
                        .child("[CHANGE]")
                        .hover(|s| s.border_color(theme.border_focused))
                        .on_click(cx.listener(|view, _event, _window, cx| {
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
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_dim)
                        .child("THRESHOLD"),
                )
                .child(
                    div()
                        .id("threshold_toggle")
                        .cursor_pointer()
                        .px_3()
                        .py_1()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .bg(theme.element_bg)
                        .text_color(theme.text_accent)
                        .border_1()
                        .border_color(theme.border)
                        .child(if app.threshold_enabled {
                            "[ON]"
                        } else {
                            "[OFF]"
                        })
                        .hover(|s| s.border_color(theme.border_focused))
                        .on_click(cx.listener(|view, _event, _window, cx| {
                            view.toggle_threshold();
                            cx.notify();
                        })),
                )
                .when(app.threshold_enabled, |this| {
                    this.child(
                        div()
                            .id("threshold_dec")
                            .when(app.config.threshold_days > 0, |d| d.cursor_pointer())
                            .px_3()
                            .py_1()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .bg(theme.element_bg)
                            .text_color(if app.config.threshold_days > 0 {
                                theme.text_accent
                            } else {
                                theme.text_dim
                            })
                            .border_1()
                            .border_color(if app.config.threshold_days > 0 {
                                theme.border
                            } else {
                                theme.border_disabled
                            })
                            .child("[-]")
                            .when(app.config.threshold_days > 0, |d| {
                                d.hover(|s| s.border_color(theme.border_focused)).on_click(
                                    cx.listener(|view, _event, _window, cx| {
                                        view.decrease_threshold();
                                        cx.notify();
                                    }),
                                )
                            }),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(theme.element_bg)
                            .border_1()
                            .border_color(theme.border)
                            .text_xs()
                            .text_color(theme.text_accent)
                            .child(format!("{:03}", app.config.threshold_days)),
                    )
                    .child(
                        div()
                            .id("threshold_inc")
                            .when(app.config.threshold_days < 365, |d| d.cursor_pointer())
                            .px_3()
                            .py_1()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .bg(theme.element_bg)
                            .text_color(if app.config.threshold_days < 365 {
                                theme.text_accent
                            } else {
                                theme.text_dim
                            })
                            .border_1()
                            .border_color(if app.config.threshold_days < 365 {
                                theme.border
                            } else {
                                theme.border_disabled
                            })
                            .child("[+]")
                            .when(app.config.threshold_days < 365, |d| {
                                d.hover(|s| s.border_color(theme.border_focused)).on_click(
                                    cx.listener(|view, _event, _window, cx| {
                                        view.increase_threshold();
                                        cx.notify();
                                    }),
                                )
                            }),
                    )
                    .child(div().text_xs().text_color(theme.text_dim).child("DAYS"))
                }),
        )
        .when(app.is_scanning, |this| {
            let theme = Theme::coder_black();
            this.child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_dim)
                                    .child("SCANNING:"),
                            )
                            .child(div().text_xs().text_color(theme.text_muted).child(
                                if app.current_scan_folder.is_empty() {
                                    "...".to_string()
                                } else {
                                    app.current_scan_folder.clone()
                                },
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .h(px(3.0))
                            .bg(theme.element_bg)
                            .border_1()
                            .border_color(theme.border)
                            .child(
                                div()
                                    .h_full()
                                    .bg(theme.text_accent)
                                    .w(relative(app.scan_progress.max(0.01))),
                            ),
                    ),
            )
        })
        .child(
            div()
                .flex()
                .gap_2()
                .child(
                    div()
                        .id("scan_button")
                        .when(!app.is_scanning, |d| d.cursor_pointer())
                        .px_3()
                        .py_1()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .bg(theme.element_bg)
                        .text_color(if app.is_scanning {
                            theme.text_dim
                        } else {
                            theme.text_accent
                        })
                        .border_1()
                        .border_color(if app.is_scanning {
                            theme.border_disabled
                        } else {
                            theme.border
                        })
                        .child(if app.is_scanning {
                            "[SCANNING...]"
                        } else {
                            "[SCAN]"
                        })
                        .when(!app.is_scanning, |d| {
                            d.hover(|s| s.border_color(theme.border_focused))
                                .on_click(cx.listener(|view, _event, _window, cx| {
                                    view.scan_for_projects();
                                    cx.notify();
                                }))
                        }),
                )
                .child(
                    div()
                        .id("delete_button")
                        .when(selected_count > 0, |d| d.cursor_pointer())
                        .px_3()
                        .py_1()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .bg(theme.element_bg)
                        .text_color(if selected_count > 0 {
                            theme.error
                        } else {
                            theme.text_dim
                        })
                        .border_1()
                        .border_color(if selected_count > 0 {
                            theme.error
                        } else {
                            theme.border_disabled
                        })
                        .child(format!("[DELETE NODE_MODULES] ({})", selected_count))
                        .when(selected_count > 0, |d| {
                            d.hover(|s| s.border_color(theme.error))
                                .on_click(cx.listener(|view, _event, _window, cx| {
                                    view.delete_selected();
                                    cx.notify();
                                }))
                        }),
                ),
        )
        .child(
            div()
                .flex()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(app.status_message.clone()),
                )
                .when(selected_count > 0, |this| {
                    this.child(
                        div()
                            .text_xs()
                            .text_color(theme.text_accent)
                            .child(format!("SELECTED: {:.2} GB", total_size_gb)),
                    )
                }),
        )
}

fn render_project_list(app: &StorageCleaner, cx: &mut Context<StorageCleaner>) -> impl IntoElement {
    let theme = Theme::coder_black();

    div()
        .id("project_list")
        .flex()
        .flex_col()
        .flex_1()
        .overflow_y_scroll()
        .p_2()
        .gap_1()
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
                    .text_xs()
                    .text_color(theme.text_dim)
                    .child("[ NO NODE_MODULES FOUND - CLICK SCAN TO START ]"),
            )
        })
}

fn render_project_card(
    project: &crate::scanner::ProjectInfo,
    index: usize,
    cx: &mut Context<StorageCleaner>,
) -> impl IntoElement {
    let theme = Theme::coder_black();
    let selected = project.selected;

    div()
        .id(("project", index))
        .flex()
        .p_2()
        .gap_2()
        .bg(theme.surface)
        .border_1()
        .border_color(if selected {
            theme.border_focused
        } else {
            theme.border
        })
        .cursor_pointer()
        .hover(|style| style.border_color(theme.border_focused))
        .on_click(cx.listener(move |view, _event, _window, cx| {
            view.toggle_project(index);
            cx.notify();
        }))
        .child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .w_4()
                .h_4()
                .border_1()
                .border_color(theme.border_focused)
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .when(selected, |this| {
                    this.bg(theme.text_accent)
                        .text_color(theme.background)
                        .child("X")
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
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text)
                                .child(
                                    project
                                        .project_path
                                        .file_name()
                                        .and_then(|n| n.to_str())
                                        .unwrap_or("unknown")
                                        .to_string()
                                        .to_uppercase(),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .gap_2()
                                .text_xs()
                                .child(
                                    div()
                                        .text_color(theme.text_dim)
                                        .child(format!("[{} DAYS]", project.days_old())),
                                )
                                .child(
                                    div()
                                        .text_color(theme.text_accent)
                                        .child(format!("[{:.2} GB]", project.size_gb())),
                                ),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_dim)
                        .child(format!("{}", project.project_path.display())),
                ),
        )
}
