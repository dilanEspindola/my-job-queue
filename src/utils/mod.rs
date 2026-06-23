use std::io;

pub fn print_status_box(active_threads: usize, tasks_executed: usize, tasks_pending: usize) {
    let width = 40;
    let border = format!("+{}+", "-".repeat(width));

    let row = |label: &str, value: usize| format!("| {:<28} {:>8} |", label, value);

    println!("{}", border);
    println!("| {:<38} |", "Job Queue Status");
    println!("{}", border);
    println!("{}", row("Active threads:", active_threads));
    println!("{}", row("Tasks executed:", tasks_executed));
    println!("{}", row("Tasks pending:", tasks_pending));
    println!("{}", border);
}

pub fn push_tasks_input() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    println!("You entered: {}", buf.trim());

    return Ok(buf.trim().to_string());
}
