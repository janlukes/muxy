use muxy_terminal::{Size, Terminal, TerminalEvent};

#[test]
fn utf8_titles_survive_every_pty_read_boundary() -> Result<(), Box<dyn std::error::Error>> {
    let title = "Příliš žluťoučký kůň — 日本語 — e\u{301} — 🦀";
    for terminator in ["\x07", "\x1b\\"] {
        let sequence = format!("\x1b]2;{title}{terminator}");
        for boundary in 0..=sequence.len() {
            let mut terminal = Terminal::new(Size { cols: 80, rows: 24 }, 1024)?;
            terminal.feed(&sequence.as_bytes()[..boundary]);
            let mut events = terminal.take_events();
            terminal.feed(&sequence.as_bytes()[boundary..]);
            events.extend(terminal.take_events());
            assert_eq!(
                events,
                [TerminalEvent::Title(title.into())],
                "boundary {boundary}"
            );
        }
        let mut terminal = Terminal::new(Size { cols: 80, rows: 24 }, 1024)?;
        let mut events = Vec::new();
        for byte in sequence.bytes() {
            terminal.feed(&[byte]);
            events.extend(terminal.take_events());
        }
        assert_eq!(events, [TerminalEvent::Title(title.into())]);
    }
    Ok(())
}
