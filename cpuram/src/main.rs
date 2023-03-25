// Code generateded by ChatGPT-3
// Create a Rust program based on Sysinfo and Tui.
// This program will display the CPU, and RAM consumption in the terminal.
// The refresh rate should be configurable from the command line.

use std::env;
use std::time::Duration;
use sysinfo::{ProcessorExt, System, SystemExt};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Gauge, Paragraph, Text};
use tui::Terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments for refresh rate
    let args: Vec<String> = env::args().collect();
    let refresh_rate = if args.len() > 1 {
        args[1].parse::<u64>().unwrap()
    } else {
        1
    };

    // Initialize Tui terminal
    let backend = CrosstermBackend::new()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Initialize Sysinfo system
    let mut system = System::new_all();

    // Main loop
    loop {
        // Update Sysinfo system
        system.refresh_all();

        // Get CPU usage and RAM usage
        let cpu_usage = system.get_processor_list()[0].get_cpu_usage();
        let ram_usage = system.get_used_memory() as f64 / system.get_total_memory() as f64;

        // Create Tui widgets
        let cpu_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("CPU Usage"))
            .gauge_style(Style::default().fg(Color::Green))
            .percent(cpu_usage as u16);
        let ram_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("RAM Usage"))
            .gauge_style(Style::default().fg(Color::Blue))
            .percent((ram_usage * 100.0) as u16);
        let system_info = Paragraph::new(vec![
            Text::raw(format!("OS: {}", system.get_os_name())),
            Text::raw(format!("Kernel: {}", system.get_kernel_version())),
            Text::raw(format!("Uptime: {}", system.get_uptime())),
        ]);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(terminal.size()?);

        // Render Tui widgets
        terminal.draw(|f| {
            f.render_widget(cpu_gauge, layout[0]);
            f.render_widget(ram_gauge, layout[1]);
            f.render_widget(system_info, layout[2]);
        })?;

        // Sleep for the specified refresh rate
        std::thread::sleep(Duration::from_secs(refresh_rate));
    }
}
