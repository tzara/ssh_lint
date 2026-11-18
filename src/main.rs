mod config;
mod models;
mod ssh;
mod worker;

use config::load_config;
use slint::Model;
use std::sync::Arc;

slint::include_modules!();

fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            for next in chars.by_ref() {
                if next.is_ascii_alphabetic() { break; }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(load_config()?);

    let servers: Vec<ServerEntry> = config.servers.iter()
        .map(|(k, v)| ServerEntry {
            key:  k.as_str().into(),
            host: v.host.as_str().into(),
        })
        .collect();

    let commands: Vec<CommandEntry> = config.commands.iter()
        .map(|(k, v)| CommandEntry {
            key:  k.as_str().into(),
            name: v.name.as_str().into(),
            desc: v.desc.as_str().into(),
        })
        .collect();

    let ui = AppWindow::new().unwrap();

    ui.set_servers(std::rc::Rc::new(slint::VecModel::from(servers)).into());
    ui.set_commands(std::rc::Rc::new(slint::VecModel::from(commands)).into());

    // Execute button
    let ui_weak = ui.as_weak();
    let cfg = config.clone();
    ui.on_execute_clicked(move || {
        let ui = match ui_weak.upgrade() { Some(u) => u, None => return };

        let sel_srv = ui.get_selected_server();
        let sel_cmd = ui.get_selected_command();
        if sel_srv < 0 || sel_cmd < 0 { return; }

        let servers  = ui.get_servers();
        let commands = ui.get_commands();

        let server_key = servers.row_data(sel_srv as usize)
            .map(|e| e.key.to_string()).unwrap_or_default();
        let cmd_key = commands.row_data(sel_cmd as usize)
            .map(|e| e.key.to_string()).unwrap_or_default();

        let host = match cfg.servers.get(&server_key)  { Some(s) => s.host.clone(), None => return };
        let cmd  = match cfg.commands.get(&cmd_key)    { Some(c) => c.cmd.clone(),  None => return };

        ui.set_running(true);
        ui.set_output_text("".into());

        let ui_out  = ui_weak.clone();
        let ui_done = ui_weak.clone();

        worker::spawn_command(
            host, cmd,
            move |output| {
                let cleaned = strip_ansi(&output);
                let handle = ui_out.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = handle.upgrade() {
                        ui.set_output_text(cleaned.into());
                    }
                });
            },
            move || {
                let handle = ui_done.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = handle.upgrade() {
                        ui.set_running(false);
                    }
                });
            },
        );
    });

    // Clear button
    let ui_weak2 = ui.as_weak();
    ui.on_clear_clicked(move || {
        if let Some(ui) = ui_weak2.upgrade() {
            ui.set_output_text("".into());
        }
    });

    ui.run().unwrap();
    Ok(())
}
