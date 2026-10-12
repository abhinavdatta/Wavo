//! The profile gate: a wall of profiles shown at app start, like browser or
//! streaming profiles. Each linked account is a profile; picking a pinned one
//! asks for its password before showing its chats.

use egui::{Align, Layout, Vec2};

use crate::app::App;
use crate::i18n::gettext;
use crate::model::{AccountId, Action};
use crate::theme::{self, Icon};

use super::widgets;

/// The password field's id, which takes the keyboard as the ask appears.
pub const PASSWORD_ID: &str = "profile-password";

/// One profile on the wall, as the switcher shows it.
struct Entry {
    id: AccountId,
    name: String,
    me: String,
    picture: Option<std::path::PathBuf>,
    pinned: bool,
    unread: u32,
}

fn entries(app: &mut App) -> Vec<Entry> {
    app.accounts
        .iter_mut()
        .map(|account| {
            let me = account.me.clone().unwrap_or_default();
            let picture = (!me.is_empty()).then(|| account.avatar(&me)).flatten();
            Entry {
                id: account.id.clone(),
                name: account.display_label(app.locale),
                me,
                picture,
                pinned: account.settings.profile_pin_hash.is_some(),
                unread: account.unread_chat_count(),
            }
        })
        .collect()
}

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    keys(app, ui.ctx());
    let palette = app.palette;
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(palette.window))
        .show(ui, |ui| {
            let rect = ui.max_rect();
            let top = theme::blend(palette.window, palette.accent, 0.10);
            widgets::paint_vertical_gradient(ui, rect, top, palette.window);
            // Centre the wall with its previous height, as the linking and
            // lock cards do; its content determines the next frame's height.
            let height_id = ui.id().with("profile-gate-height");
            let known_height = ui
                .ctx()
                .data(|data| data.get_temp::<f32>(height_id))
                .unwrap_or(360.0);
            let block_height = (known_height.min(rect.height() - 24.0)).max(0.0);
            let block =
                egui::Rect::from_center_size(rect.center(), Vec2::new(rect.width(), block_height));
            let mut block_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(block)
                    .layout(Layout::top_down(Align::Center)),
            );
            let asking = app
                .profile_gate
                .as_ref()
                .and_then(|gate| gate.asking.clone());
            theme::text(&mut block_ui, "WAVO", theme::semibold(30.0), palette.text);
            let label = match (
                app.profile_gate.as_ref().map(|gate| gate.asking.is_some()),
                app.accounts.len(),
            ) {
                (Some(true), _) => gettext(app.locale, "Enter the profile password"),
                (_, 1) => gettext(app.locale, "This profile is protected"),
                _ => gettext(app.locale, "Who's watching?"),
            };
            block_ui.add_space(8.0);
            theme::paragraph(
                &mut block_ui,
                label,
                theme::regular(14.0),
                palette.secondary,
            );
            block_ui.add_space(20.0);
            match asking {
                Some(id) => password_ask(app, &mut block_ui, &id),
                None => wall(app, &mut block_ui),
            }
            let height = block_ui.min_rect().height();
            if (height - known_height).abs() > 2.0 {
                ui.ctx()
                    .data_mut(|data| data.insert_temp(height_id, height.round()));
                ui.ctx().request_repaint();
            }
        });
}

fn keys(app: &mut App, ctx: &egui::Context) {
    let asking = app
        .profile_gate
        .as_ref()
        .is_some_and(|gate| gate.asking.is_some());
    ctx.input_mut(|input| {
        if asking {
            if input.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                app.actions.push(Action::LeaveProfileGate);
            }
        } else if input
            .consume_key(egui::Modifiers::NONE, egui::Key::Escape)
            .then_some(())
            .is_some()
        {
            // Nothing to leave: the wall is the top of the app at start.
        }
    });
}

fn wall(app: &mut App, ui: &mut egui::Ui) {
    let accounts = entries(app);
    let size = 96.0;
    // A horizontal child Ui starts at the left edge, so lead it by hand to
    // put the wall in the middle of the screen. Each tile is one fixed
    // width, plus the spacing between them.
    let count = accounts.len() + 1;
    let row = count as f32 * (size + 24.0) + count.saturating_sub(1) as f32 * 20.0;
    ui.horizontal(|ui| {
        ui.add_space(((ui.available_width() - row) / 2.0).max(0.0));
        ui.spacing_mut().item_spacing.x = 20.0;
        for entry in &accounts {
            profile_tile(app, ui, entry, size);
        }
        add_tile(app, ui, size);
    });
}

fn profile_tile(app: &mut App, ui: &mut egui::Ui, entry: &Entry, size: f32) {
    let palette = app.palette;
    ui.vertical(|ui| {
        ui.set_width(size + 24.0);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                true,
                format!(
                    "{} ({})",
                    entry.name,
                    if entry.pinned { "pinned" } else { "open" }
                ),
            )
        });
        if response.hovered() {
            ui.painter()
                .rect_filled(rect.expand(4.0), 12.0, palette.surface_hover);
        }
        let center = egui::pos2(rect.center().x, rect.center().y - 2.0);
        widgets::paint_avatar(
            ui,
            &palette,
            egui::Rect::from_center_size(center, Vec2::splat(size - 8.0)),
            &entry.name,
            if entry.me.is_empty() {
                entry.id.as_str()
            } else {
                &entry.me
            },
            entry.picture.as_deref(),
        );
        if entry.pinned {
            // A lock badge on the bottom right says this one asks a password.
            let badge = 20.0;
            let at = egui::pos2(rect.right() - badge / 2.0, rect.bottom() - badge / 2.0);
            let badge_rect = egui::Rect::from_center_size(at, Vec2::splat(badge + 6.0));
            ui.painter()
                .circle_filled(at, badge / 2.0 + 2.0, palette.panel);
            ui.painter().circle_filled(
                at,
                badge / 2.0,
                theme::blend(palette.panel, palette.text, 0.06),
            );
            theme::paint_icon(ui, Icon::Lock, badge_rect, 12.0, palette.secondary);
        }
        if entry.unread > 0 {
            let at = egui::pos2(rect.right() - 4.0, rect.top() + 4.0);
            ui.painter().circle_filled(at, 7.0, palette.accent);
        }
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.horizontal(|ui| {
                if widgets::rich_text(ui, &entry.name, theme::semibold(13.5), palette.text)
                    .hovered()
                {}
            });
            ui.add_space(6.0);
        });
        if response.clicked() {
            app.actions.push(Action::PickProfile(entry.id.clone()));
        }
        profile_menu(app, &response, &entry.id, &entry.name);
    });
}

/// A right-click menu on a profile tile: open it, or remove it (with the
/// confirm dialog, as Settings does). Never on the account on screen, whose
/// exit is the app lock's unlink.
fn profile_menu(app: &mut App, response: &egui::Response, id: &AccountId, name: &str) {
    let palette = app.palette;
    let on_screen = &app.account().id == id;
    egui::Popup::context_menu(response)
        .frame(widgets::menu_frame(&palette))
        .show(|ui| {
            if widgets::menu_item(
                ui,
                &palette,
                Some(Icon::ChevronRight),
                &gettext(app.locale, "Open"),
            ) {
                app.actions.push(Action::PickProfile(id.clone()));
            }
            if !on_screen {
                widgets::menu_separator(ui, &palette);
                if widgets::menu_item(
                    ui,
                    &palette,
                    Some(Icon::Trash),
                    &gettext(app.locale, "Remove {name}…").replace("{name}", name),
                ) {
                    app.actions.push(Action::ShowDialog(
                        crate::model::Dialog::ConfirmRemoveAccount(id.clone()),
                    ));
                }
            }
        });
}

fn add_tile(app: &mut App, ui: &mut egui::Ui, size: f32) {
    let palette = app.palette;
    ui.vertical(|ui| {
        ui.set_width(size + 24.0);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "Add profile")
        });
        theme::reveal_focus(&response);
        let fill = if response.hovered() {
            palette.surface_hover
        } else {
            palette.surface
        };
        ui.painter().rect_filled(rect.shrink(4.0), 10.0, fill);
        theme::paint_icon(
            ui,
            Icon::Plus,
            rect,
            size * 0.4,
            if response.hovered() {
                palette.text
            } else {
                palette.secondary
            },
        );
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            theme::text(
                ui,
                gettext(app.locale, "Add profile"),
                theme::semibold(13.5),
                palette.secondary,
            );
        });
        if response.clicked() {
            app.actions.push(Action::AddAccount);
        }
    });
}

/// The password ask for a picked, pinned profile.
fn password_ask(app: &mut App, ui: &mut egui::Ui, id: &AccountId) {
    let palette = app.palette;
    let locale = app.locale;
    let Some(account) = app.accounts.iter_mut().find(|account| account.id == *id) else {
        app.actions.push(Action::LeaveProfileGate);
        return;
    };
    let name = account.display_label(locale);
    let me = account.me.clone().unwrap_or_default();
    let picture = (!me.is_empty()).then(|| account.avatar(&me)).flatten();
    // Same as the wall: lead the column by hand so it sits centred, since a
    // child Ui starts at the left edge of its parent.
    let width = 360.0_f32.min(ui.available_width());
    ui.horizontal(|ui| {
        ui.add_space(((ui.available_width() - width) / 2.0).max(0.0));
        ui.vertical(|ui| {
            ui.set_width(width);
            // paint_avatar wants absolute screen coordinates: allocate the
            // avatar through the layout, which hands one over.
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::splat(72.0), egui::Sense::hover());
                widgets::paint_avatar(
                    ui,
                    &palette,
                    rect,
                    &name,
                    if me.is_empty() { id.as_str() } else { &me },
                    picture.as_deref(),
                );
            });
            ui.add_space(4.0);
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                theme::text(ui, &name, theme::semibold(15.0), palette.text);
            });
            ui.add_space(12.0);
            let gate = app.profile_gate.as_mut().expect("gate open");
            let busy = gate.busy;
            let id_field = egui::Id::new(PASSWORD_ID);
            let mut submit = ui.memory(|memory| memory.has_focus(id_field))
                && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
            let field = ui.add_enabled(
                !busy,
                egui::TextEdit::singleline(&mut gate.entry)
                    .id(id_field)
                    .password(true)
                    .hint_text(gettext(locale, "Profile password"))
                    .desired_width(240.0_f32.min(ui.available_width())),
            );
            if ui.memory(|memory| memory.focused().is_none()) && !busy {
                field.request_focus();
            }
            if let Some(wait) = gate.wait_left() {
                let seconds = wait.as_secs_f32().ceil().max(1.0) as u64;
                theme::paragraph(
                    ui,
                    gettext(
                        locale,
                        "Too many wrong passwords. Try again in {seconds} s.",
                    )
                    .replace("{seconds}", &seconds.to_string()),
                    theme::regular(13.0),
                    palette.danger,
                );
                ui.ctx()
                    .request_repaint_after(wait.min(std::time::Duration::from_millis(250)));
            } else if gate.wrong {
                theme::paragraph(
                    ui,
                    gettext(locale, "Wrong password. Try again."),
                    theme::regular(13.0),
                    palette.danger,
                );
            }
            ui.add_space(4.0);
            if busy {
                theme::spinner(ui, 20.0, palette.accent);
            } else {
                submit |= ui
                    .add_enabled_ui(gate.can_try(), |ui| {
                        theme::pill_button(ui, &palette, &gettext(locale, "Open"), true)
                    })
                    .inner
                    .clicked();
            }
            if submit {
                app.actions.push(Action::SubmitProfilePassword);
            }
            ui.add_space(8.0);
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                if theme::link(
                    ui,
                    gettext(locale, "Back to profiles"),
                    theme::regular(13.0),
                    palette.accent,
                )
                .clicked()
                {
                    app.actions.push(Action::LeaveProfileGate);
                }
            });
        });
    });
}

#[cfg(test)]
mod layout_tests {
    use crate::app::{App, ProfileGate};
    use crate::model::AccountId;
    use crate::paths::AppDirs;
    use crate::settings::Settings;

    const WINDOW: egui::Vec2 = egui::vec2(1180.0, 780.0);

    /// One frame of the complete app, keeping the painted shapes.
    fn frame(app: &mut App, ctx: &egui::Context) -> Vec<egui::epaint::ClippedShape> {
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, WINDOW)),
                ..Default::default()
            },
            |ui| app.frame_ui(ui),
        );
        output.textures_delta.clear();
        output.shapes
    }

    /// The centre of every doodle face card the wall paints for a profile
    /// without a picture; the tiles they belong to are centred on them.
    fn face_cards(shapes: &[egui::epaint::ClippedShape]) -> Vec<egui::Pos2> {
        fn scan(shape: &egui::Shape, found: &mut Vec<egui::Pos2>) {
            match shape {
                egui::Shape::Circle(circle)
                    if circle.fill == egui::Color32::WHITE
                        && (30.0..50.0).contains(&circle.radius) =>
                {
                    found.push(circle.center);
                }
                egui::Shape::Vec(children) => {
                    for child in children {
                        scan(child, found);
                    }
                }
                _ => {}
            }
        }
        let mut found = Vec::new();
        for clipped in shapes {
            scan(&clipped.shape, &mut found);
        }
        found
    }

    /// The password ask for a picked profile sits centred too: it replaced
    /// the wall once and drew against the left edge.
    #[test]
    fn the_password_ask_is_centred_in_the_window() {
        let root = std::env::temp_dir().join(format!(
            "wavo-profile-ask-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let (mut app, _events) = App::headless(AppDirs::under(&root), Settings::default());
        let ctx = egui::Context::default();
        app.attach(&ctx);
        app.accounts[0].settings.profile_pin_hash = Some(crate::app_lock::verifier("1234"));
        app.profile_gate = Some(ProfileGate::default());
        if let Some(gate) = &mut app.profile_gate {
            gate.asking = Some(AccountId::first());
        }

        let mut shapes = Vec::new();
        for _ in 0..5 {
            shapes = frame(&mut app, &ctx);
        }

        let cards = face_cards(&shapes);
        assert!(!cards.is_empty(), "the ask paints its profile avatar");
        for card in cards {
            assert!(
                (WINDOW.x * 0.35..WINDOW.x * 0.65).contains(&card.x),
                "the ask's avatar sits at x={}, far from the middle of a {}-wide window",
                card.x,
                WINDOW.x
            );
            assert!(
                (WINDOW.y * 0.25..WINDOW.y * 0.75).contains(&card.y),
                "the ask's avatar sits at y={}, far from the middle of a {}-tall window",
                card.y,
                WINDOW.y
            );
        }
    }

    /// The wall of profiles was drawn against the top-left corner, with the
    /// rest of the window empty; it belongs in the middle of the screen.
    #[test]
    fn the_profile_wall_is_centred_in_the_window() {
        let root = std::env::temp_dir().join(format!(
            "wavo-profile-gate-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let (mut app, _events) = App::headless(AppDirs::under(&root), Settings::default());
        let ctx = egui::Context::default();
        app.attach(&ctx);
        app.profile_gate = Some(ProfileGate::default());

        // The first frame remembers the wall's height; the next ones centre
        // the block with it, as the running app does.
        let mut shapes = Vec::new();
        for _ in 0..5 {
            shapes = frame(&mut app, &ctx);
        }

        let cards = face_cards(&shapes);
        assert!(!cards.is_empty(), "the wall paints its profile tiles");
        for card in cards {
            assert!(
                (WINDOW.x * 0.35..WINDOW.x * 0.65).contains(&card.x),
                "a profile tile sits at x={}, far from the middle of a {}-wide window",
                card.x,
                WINDOW.x
            );
            assert!(
                (WINDOW.y * 0.30..WINDOW.y * 0.75).contains(&card.y),
                "a profile tile sits at y={}, far from the middle of a {}-tall window",
                card.y,
                WINDOW.y
            );
        }
    }
}
