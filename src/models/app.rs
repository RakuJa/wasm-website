use crate::models::item::Item;
use crate::{
    ALT_ROW_BG_COLOR, COMPLETED_TEXT_FG_COLOR, NORMAL_ROW_BG, SELECTED_STYLE, TEXT_FG_COLOR,
    TODO_HEADER_STYLE,
};
use ratzilla::ratatui::buffer::Buffer;
use ratzilla::ratatui::layout::{Constraint, Layout, Rect};
use ratzilla::ratatui::prelude::{Color, Line, StatefulWidget, Stylize, Widget};
use ratzilla::ratatui::symbols;
use ratzilla::ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, Wrap,
};
use ratzilla::utils::open_url;

#[derive(Default)]
pub struct App {
    should_exit: bool,
    todo_list: BulletPoints,
}

struct BulletPoints {
    items: Vec<BulletItem>,
    state: ListState,
}

#[derive(Debug)]
struct BulletItem {
    item: Item,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Todo,
    Completed,
}

impl Status {
    pub const fn get_status_char(self) -> char {
        match self {
            Self::Todo => '☐',
            Self::Completed => '✓',
        }
    }
}

impl FromIterator<(Status, Item)> for BulletPoints {
    fn from_iter<I: IntoIterator<Item = (Status, Item)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, item)| BulletItem::new(status, item))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl Default for BulletPoints {
    fn default() -> Self {
        Item::get_list_of_titles()
            .into_iter()
            .map(|title| (Status::Todo, Item { title }))
            .collect()
    }
}

impl BulletItem {
    const fn new(status: Status, item: Item) -> Self {
        Self { item, status }
    }
}

impl App {
    pub fn on_down(&mut self) {
        self.select_next();
    }

    pub fn on_up(&mut self) {
        self.select_previous();
    }

    pub fn on_right(&mut self) {
        self.toggle_status();
    }

    pub fn on_left(&mut self) {
        self.select_none();
    }

    pub fn on_enter(&mut self) {
        self.on_right();
    }

    pub fn open_link(&self) {
        if let Some(index) = self.todo_list.state.selected() {
            if let Some(bul_item) = self.todo_list.items.get(index) {
                let url = bul_item.item.get_link();
                if !url.is_empty() {
                    let _ = open_url(url.as_str(), true);
                }
            }
        }
    }
    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => self.should_exit = true,
            'w' | '↑' => self.on_up(),
            'a' | '←' => self.on_left(),
            's' | '↓' => self.on_down(),
            'd' => self.on_right(),
            'h' => self.select_first(),
            'e' => self.select_last(),

            _ => {}
        }
    }

    fn select_none(&mut self) {
        self.todo_list.state.select(None);
    }

    fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.todo_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.todo_list.state.select_last();
    }

    /// Changes the status of the selected list item
    fn toggle_status(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list.items[i].status = match self.todo_list.items[i].status {
                Status::Completed => Status::Todo,
                Status::Todo => Status::Completed,
            }
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Daniele's public data")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ or ws to move, ← or a to unselect, → or d to change status, h/e to go top/bottom, CTRL + Enter to open link.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Topics").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .todo_list
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = self.todo_list.state.selected().map_or_else(
            || "Nothing selected...".to_string(),
            |i| {
                let char_status = self.todo_list.items[i].status.get_status_char();
                format!(
                    "{} Data:\n{}",
                    char_status,
                    self.todo_list.items[i].item.get_description()
                )
            },
        );

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("Details").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&BulletItem> for ListItem<'_> {
    fn from(value: &BulletItem) -> Self {
        let line = match value.status {
            Status::Todo => Line::styled(format!(" ☐ {}", value.item.title), TEXT_FG_COLOR),
            Status::Completed => {
                Line::styled(format!(" ✓ {}", value.item.title), COMPLETED_TEXT_FG_COLOR)
            }
        };
        ListItem::new(line)
    }
}
