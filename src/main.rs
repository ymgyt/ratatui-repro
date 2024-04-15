use ratatui::{
    layout::Constraint,
    text::Span,
    widgets::{Row, Table, Widget as _},
};

fn main() {
    let backend = ratatui::backend::CrosstermBackend::new(std::io::stdout());
    let mut terminal = ratatui::Terminal::new(backend).unwrap();

    terminal.draw(|frame| {

        let rows = vec![
            Row::new([Span::from("🦀 RFC8628 OAuth 2.0 Device Authorization GrantでCLIからGithubのaccess tokenを取得する")])
        ];
        let widths = [Constraint::Length(83)];
        let table = Table::new(rows, widths);
        table.render(frame.size(), frame.buffer_mut());
    }).unwrap();
}
