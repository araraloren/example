use std::io;
use std::sync::Arc;

use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::prelude::*;
use ratatui::widgets::*;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;

use httping::PingServer;
use httping::Task;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DisplayStyle {
    Table,
    Total,
    Chart(usize),
}

pub const fn cons_min(val: u16) -> Constraint {
    Constraint::Min(val)
}

pub const fn cons_length(val: u16) -> Constraint {
    Constraint::Length(val)
}

pub const fn cons_percentage(val: u16) -> Constraint {
    Constraint::Percentage(val)
}

macro_rules! extract {
    ($($var:ident),+ <- $what:ident[$beg:literal .. $end:literal]) => {
        let [$($var),+] = $what[$beg..$end] else {
            panic!(concat!("invalid range operator on ", stringify!($what)))
        };
    };
}

pub struct App {
    cache: String,
    editing: bool,
    runtime: Runtime,
    display_style: DisplayStyle,
    task_index: ListState,
    task_list: Vec<Task>,
    server_index: ListState,
    server_list: Vec<Arc<dyn PingServer + Send + Sync>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            server_list: vec![],
            server_index: ListState::default(),
            task_list: vec![],
            task_index: ListState::default(),
            cache: String::default(),
            editing: false,
            display_style: DisplayStyle::Table,
            runtime: Builder::new_multi_thread().build().unwrap(),
        }
    }
}

impl App {
    pub fn with_server(mut self, server: impl PingServer + Send + Sync + 'static) -> Self {
        self.server_list.push(Arc::new(server));
        self
    }

    pub fn add_server(&mut self, server: impl PingServer + Send + Sync + 'static) -> &mut Self {
        self.server_list.push(Arc::new(server));
        self
    }

    pub fn ping_host(&mut self, server_index: usize, host: String) -> &mut Self {
        let server = self.server_list[server_index].clone();
        let (resp_tx, resp_rx) = tokio::sync::mpsc::channel(128);
        let (cc_tx, cc_rx) = tokio::sync::oneshot::channel();
        let target = host.clone();
        let handler = self
            .runtime
            .spawn(async move { server.ping(target, cc_rx, resp_tx).await });

        self.task_list
            .push(Task::new(host, handler, cc_tx, resp_rx));
        self
    }
}

impl App {
    pub fn view(&mut self, frame: &mut Frame) {
        let layout =
            Layout::vertical([cons_length(3), cons_min(3), cons_length(3)]).split(frame.size());

        extract!(title_layout, main_layout, status_layout <- layout[0..3]);

        let layout =
            Layout::horizontal([cons_percentage(30), cons_percentage(70)]).split(main_layout);

        extract!(op_layout, info_layout <- layout[0..2]);

        let layout = Layout::vertical([cons_percentage(70), cons_percentage(30)])
            .margin(2)
            .split(op_layout);

        extract!(task_layout, input_layout <- layout[0..2]);

        let layout = Layout::vertical([cons_min(3), cons_length(3)]).split(input_layout);

        extract!(server_layout, text_layout <- layout[0..2]);

        let task_list = List::new(
            self.task_list
                .iter()
                .map(|task| Text::from(task.host().to_owned()).centered())
                .collect::<Vec<_>>(),
        )
        .block(
            Block::bordered()
                .title("Task")
                .title_alignment(Alignment::Center),
        )
        .highlight_spacing(HighlightSpacing::Always)
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
        .highlight_style(
            Style::default()
                .bg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        );

        if !self.task_list.is_empty() && self.task_index.selected().is_none() {
            self.task_index = ListState::default().with_selected(Some(0));
        }

        frame.render_stateful_widget(task_list, task_layout, &mut self.task_index);

        let server_list = List::new(
            self.server_list
                .iter()
                .map(|server| Text::from(server.name().to_owned()).centered())
                .collect::<Vec<_>>(),
        )
        .block(
            Block::bordered()
                .title("Server")
                .title_alignment(Alignment::Center),
        )
        .highlight_spacing(HighlightSpacing::Always)
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
        .highlight_style(
            Style::default()
                .bg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        );

        if !self.server_list.is_empty() && self.server_index.selected().is_none() {
            self.server_index = ListState::default().with_selected(Some(0));
        }

        frame.render_stateful_widget(server_list, server_layout, &mut self.server_index);

        let input = Paragraph::new(self.cache.clone()).block(
            Block::bordered()
                .title("Input")
                .title_alignment(Alignment::Center),
        );

        frame.render_widget(input, text_layout);
        if self.editing {
            frame.set_cursor(
                text_layout.x + (self.cache.len() as u16 + 1).min(text_layout.width),
                text_layout.y + 1,
            );
        }
    }

    pub fn update(&mut self, event: Event) -> io::Result<bool> {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                if !self.editing {
                    match key.code {
                        KeyCode::Char('e') => {
                            self.editing = true;
                        }
                        KeyCode::Esc => return Ok(true),
                        KeyCode::Down => {
                            self.task_index.select_next();
                        }
                        KeyCode::Up => {
                            self.task_index.select_previous();
                        }
                        KeyCode::Left => {
                            self.server_index.select_previous();
                        }
                        KeyCode::Right => {
                            self.server_index.select_next();
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Backspace => {
                            self.cache.pop();
                        }
                        KeyCode::Enter => {
                            if let Some(selected) = self.server_index.selected() {
                                self.ping_host(selected, self.cache.clone());
                                self.cache.clear();
                                self.editing = false;
                            }
                        }
                        KeyCode::Char(c) => {
                            self.cache.push(c);
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(false)
    }
}
