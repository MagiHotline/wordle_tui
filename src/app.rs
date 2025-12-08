use color_eyre::Result;
use crossterm::event::KeyModifiers;
use itertools::Itertools;
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Margin, Rect}, style::{self, Color, Modifier, Style, Stylize}, text::Text, widgets::{
        Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState, Widget,
    }
};
use style::palette::tailwind;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use wordle_tui::WordleBox;
use std::io;

const PALETTES: [tailwind::Palette; 3] = [
    tailwind::GRAY,
    tailwind::GREEN,
    tailwind::YELLOW
];

const INFO_TEXT: &str = "(Esc) quit | (←) move left | (→) move right";

const ITEM_HEIGHT: usize = 6;

struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}


#[derive(Debug, Default)]
pub struct Data {
    input: [[WordleBox; 5]; 6],
    solution: String
}

pub struct App {
    state: TableState,
    cell_size: usize,
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}


impl App {

    fn new() -> Self {
        let data_vec : [WordleBox; 5] = [
            WordleBox::new(' ', wordle_tui::Color::Gray); 5];
        Self {
            state: TableState::default().with_selected(0),
            cell_size: ' '.width().unwrap_or(0),
            scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0
        }
    }

    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
    }


    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('l') | KeyCode::Right => self.next_column(),
                        KeyCode::Char('h') | KeyCode::Left => self.previous_column(),
                        _ => {}
                    }
                }
            }
        }
    }

    /*
    fn draw(&mut self, frame: &mut Frame) {
         let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
         let rects = vertical.split(frame.area());

         self.set_colors();

         self.render_table(frame, rects[0]);
         self.render_scrollbar(frame, rects[0]);
         self.render_footer(frame, rects[1]);
     }

     fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);
        let selected_col_style = Style::default().fg(self.colors.selected_column_style_fg);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        let header = ["Name", "Address", "Email"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(4)
        });
        let bar = " █ ";
                let t = Table::new(
                    rows,
                    [
                        // + 1 is for padding.
                        Constraint::Length(self.longest_item_lens.0 + 1),
                        Constraint::Min(self.longest_item_lens.1 + 1),
                        Constraint::Min(self.longest_item_lens.2),
                    ],
                )
                .header(header)
                .row_highlight_style(selected_row_style)
                .column_highlight_style(selected_col_style)
                .cell_highlight_style(selected_cell_style)
                .highlight_symbol(Text::from(vec![
                    "".into(),
                    bar.into(),
                    bar.into(),
                    "".into(),
                ]))
                .bg(self.colors.buffer_bg)
                .highlight_spacing(HighlightSpacing::Always);
                frame.render_stateful_widget(t, area, &mut self.state);
        }


        */

}
