//! Tray icon: quiet, or problem (daemon down or signatures older than seven days).
//! The attention state (unread first-seen events, icons/tray-attention.png) arrives
//! with activity collection. Menu: Open, Pause notifications, Quit.
use crate::daemon::Launcher;
use bigman_core::{
    ipc::{Request, Response},
    model::now,
};
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TrayState {
    Quiet,
    Problem,
}

/// Until when desktop notifications are paused (Unix seconds).
#[derive(Default)]
pub struct NotificationPause(pub AtomicU64);

const QUIET: &[u8] = include_bytes!("../icons/tray-quiet.png");
const PROBLEM: &[u8] = include_bytes!("../icons/tray-problem.png");
const SIGNATURES_MAX_AGE: u64 = 7 * 86_400;

fn icon(state: TrayState) -> tauri::Result<Image<'static>> {
    Image::from_bytes(match state {
        TrayState::Quiet => QUIET,
        TrayState::Problem => PROBLEM,
    })
}

fn tooltip(state: TrayState) -> &'static str {
    match state {
        TrayState::Quiet => "Bigman",
        TrayState::Problem => "Bigman: background collection needs attention",
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
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &pause, &quit])?;
    let tray = TrayIconBuilder::with_id("bigman")
        .icon(icon(TrayState::Quiet)?)
        .tooltip(tooltip(TrayState::Quiet))
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "pause" => app
                .state::<NotificationPause>()
                .0
                .store(now() + 3_600, Ordering::Relaxed),
            "quit" => app.exit(0),
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
            std::thread::sleep(Duration::from_secs(30));
        }
    });
    Ok(())
}

fn current_state(launcher: &Launcher) -> TrayState {
    match launcher.request(Request::Status) {
        Ok(Response::Status(status)) => {
            let stale = status
                .signatures
                .is_none_or(|s| now().saturating_sub(s.updated) > SIGNATURES_MAX_AGE);
            if stale {
                TrayState::Problem
            } else {
                TrayState::Quiet
            }
        }
        _ => TrayState::Problem,
    }
}
