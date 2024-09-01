use crossterm::execute;
use crossterm::cursor;

pub fn set_cursor(stdout: &mut std::io::Stdout, x: u16, y: u16) -> std::io::Result<()> {
    execute!(stdout, cursor::MoveTo(x, y))?;
    Ok(())
}

pub fn hide_cursor(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    execute!(stdout, cursor::Hide)?;
    Ok(())
}

pub fn show_cursor(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    execute!(stdout, cursor::Show)?;
    Ok(())
}

pub fn print(stdout: &mut std::io::Stdout, text: &str, x: Option<u16>, y: Option<u16>) -> std::io::Result<()> {
    if let Some(x) = x {
        if let Some(y) = y {
            set_cursor(stdout, x, y)?;
        }
    }
    execute!(stdout, crossterm::style::Print(text))?;
    Ok(())
}

pub fn clear(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
    Ok(())
}

pub fn clear_line(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine))?;
    Ok(())
}
