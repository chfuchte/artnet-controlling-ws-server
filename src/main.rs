pub mod app;
pub mod artnet;
pub mod utils;

use std::borrow::BorrowMut;

use crossterm::cursor;
use crossterm::event;
use crossterm::execute;
use crossterm::terminal;

use app::main_loop;

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();

    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
        terminal::SetTitle("ArtNet Controller"),
        event::EnableBracketedPaste,
        event::DisableFocusChange,
        cursor::MoveTo(0, 0),
        cursor::EnableBlinking,
        cursor::SetCursorStyle::BlinkingBlock,
        cursor::Show,
    )?;
    // as this throws in PowerShell, we need to catch it
    let _ = execute!(stdout, event::DisableMouseCapture);

    main_loop(stdout.borrow_mut())?;

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
