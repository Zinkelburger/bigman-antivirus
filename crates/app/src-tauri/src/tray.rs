//! Tray icon: quiet; attention when Activity has unread entries; problem when the
//! daemon is down or signatures are older than seven days. A left click opens the
//! window. The right-click menu: Open, Pause notifications (a daemon setting, since
//! the daemon raises them), and two ways to quit, because bigmand outlives the
//! window unless asked to stop. Closing the window leaves the app here when the
//! desktop shows tray icons.
use crate::daemon::Launcher;
use bigman_core::{
    ipc::{Request, Response},
    model::now,
};
use std::{
    process::{Command, Stdio},
    sync::Arc,
    time::Duration,
};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TrayState {
    Quiet,
    Attention,
    Problem,
}

const QUIET: &[u8] = include_bytes!("../icons/tray-quiet.png");
const ATTENTION: &[u8] = include_bytes!("../icons/tray-attention.png");
const PROBLEM: &[u8] = include_bytes!("../icons/tray-problem.png");
const SIGNATURES_MAX_AGE: u64 = 7 * 86_400;

fn icon(state: TrayState) -> tauri::Result<Image<'static>> {
    Image::from_bytes(match state {
        TrayState::Quiet => QUIET,
        TrayState::Attention => ATTENTION,
        TrayState::Problem => PROBLEM,
    })
}

/// The line under the name "Bigman" (the tray item's title) when the pointer rests on it.
fn tooltip(state: TrayState) -> &'static str {
    match state {
        TrayState::Quiet => "Nothing new in Activity",
        TrayState::Attention => "New entries in Activity",
        TrayState::Problem => "Background collection needs attention",
    }
}

pub fn install(app: &AppHandle) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "Open Bigman", true, None::<&str>)?;
    let pause = MenuItem::with_id(
        app,
        "pause",
        "Pause notifications for 1 hour",
        true,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(
        app,
        "quit",
        "Quit, keep background checks running",
        true,
        None::<&str>,
    )?;
    let quit_all = MenuItem::with_id(
        app,
        "quit_all",
        "Quit and stop background checks",
        true,
        None::<&str>,
    )?;
    let menu = Menu::with_items(app, &[&open, &pause, &quit, &quit_all])?;
    let tray = TrayIconBuilder::with_id("bigman")
        .icon(icon(TrayState::Quiet)?)
        // The name the desktop lists this icon under, and the tooltip's first line.
        .title("Bigman")
        .tooltip(tooltip(TrayState::Quiet))
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            } = event
            {
                crate::show_window(tray.app_handle());
            }
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => crate::show_window(app),
            "pause" => {
                let launcher = app.state::<Arc<Launcher>>().inner().clone();
                std::thread::spawn(move || pause_notifications(&launcher));
            }
            "quit" => app.exit(0),
            "quit_all" => {
                let launcher = app.state::<Arc<Launcher>>().inner().clone();
                let app = app.clone();
                std::thread::spawn(move || {
                    launcher.stop_daemon();
                    app.exit(0);
                });
            }
            _ => {}
        })
        .build(app)?;

    // Poll the daemon in the background; the icon changes only when the state does.
    let launcher = app.state::<Arc<Launcher>>().inner().clone();
    std::thread::spawn(move || {
        let mut shown = TrayState::Quiet;
        loop {
            let state = current_state(&launcher);
            if state != shown && tray.set_icon(icon(state).ok()).is_ok() {
                let _ = tray.set_tooltip(Some(tooltip(state)));
                shown = state;
            }
            std::thread::sleep(Duration::from_secs(15));
        }
    });
    Ok(())
}

/// Whether this desktop shows tray icons: something owns the StatusNotifierItem
/// watcher name, as KDE Plasma does and GNOME does with the AppIndicator extension.
/// Asked through `gdbus`, which ships with GLib. No answer counts as no tray, so a
/// closed window is never left running where nothing on screen leads back to it.
pub fn desktop_has_tray() -> bool {
    Command::new("gdbus")
        .args([
            "call",
            "--session",
            "--timeout",
            "2",
            "--dest",
            "org.freedesktop.DBus",
            "--object-path",
            "/org/freedesktop/DBus",
            "--method",
            "org.freedesktop.DBus.NameHasOwner",
            "org.kde.StatusNotifierWatcher",
        ])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .is_ok_and(|out| out.status.success() && out.stdout.starts_with(b"(true"))
}

fn pause_notifications(launcher: &Launcher) {
    if let Ok(Response::Notifications(mut settings)) = launcher.request(Request::Notifications) {
        settings.paused_until = Some(now() + 3_600);
        let _ = launcher.request(Request::SetNotifications { settings });
    }
}

fn current_state(launcher: &Launcher) -> TrayState {
    let Ok(Response::Status(status)) = launcher.request(Request::Status) else {
        return TrayState::Problem;
    };
    let stale = status
        .signatures
        .is_none_or(|s| now().saturating_sub(s.updated) > SIGNATURES_MAX_AGE);
    match launcher.request(Request::Unread) {
        _ if stale => TrayState::Problem,
        Ok(Response::Unread(u)) if u.first_seen + u.to_review > 0 => TrayState::Attention,
        _ => TrayState::Quiet,
    }
}
