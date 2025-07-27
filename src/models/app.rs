use crate::models::status::Status;
use crate::models::topic::Topic;
use crate::{NORMAL_BG, SELECTED_STYLE, TEXT_DATA_COLOR, TODO_HEADER_STYLE};
use ratzilla::ratatui::buffer::Buffer;
use ratzilla::ratatui::layout::{Constraint, Layout, Rect};
use ratzilla::ratatui::prelude::{Line, StatefulWidget, Style, Stylize, Widget};
use ratzilla::ratatui::symbols;
use ratzilla::ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, Wrap,
};
use ratzilla::utils::open_url;
use strum::IntoEnumIterator;

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
    topic: Topic,
    status: Status,
}

impl FromIterator<(Status, Topic)> for BulletPoints {
    fn from_iter<I: IntoIterator<Item = (Status, Topic)>>(iter: I) -> Self {
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
        Topic::iter().map(|title| (Status::Todo, title)).collect()
    }
}

impl BulletItem {
    const fn new(status: Status, item: Topic) -> Self {
        Self {
            topic: item,
            status,
        }
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
                let url = bul_item.topic.get_link();
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
            .light_magenta()
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ or ws to move, ← or a to unselect, → or d to change status, h/e to go top/bottom, CTRL + Enter to open link.")
            .light_magenta()
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Topics").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .todo_list
            .items
            .iter()
            //.enumerate()
            .map(|todo_item| {
                //let color = alternate_colors(i);
                ListItem::from(todo_item) //.bg(color)
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
                let item = &self.todo_list.items[i];
                let topic = item.topic.clone();
                let descr = topic.get_description(item.status);
                let command = topic.to_string().to_ascii_lowercase();
                format!("visitor@danielegiachetto.com:$ ~ {command}:\n{descr}")
            },
        );

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("Terminal").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_DATA_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

impl From<&BulletItem> for ListItem<'_> {
    fn from(value: &BulletItem) -> Self {
        let status = value.status;
        let style = Style::new().italic().fg(status.get_status_color());
        ListItem::new(Line::styled(
            format!(" {} {}", status.get_status_char(), value.topic),
            style,
        ))
    }
}
