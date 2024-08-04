use std::io::Write;
use std::sync::Arc;

use httping::Ui;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::prelude::*;
use ratatui::widgets::*;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;

use httping::PingServer;
use httping::Task;
use tracing::debug;

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
    resp_index: TableState,
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
            resp_index: TableState::default(),
            display_style: DisplayStyle::Table,
            runtime: Builder::new_multi_thread().enable_all().build().unwrap(),
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
            Layout::horizontal([cons_percentage(50), cons_percentage(50)]).split(status_layout);

        extract!(status_layout, help_layout <- layout[0..2]);

        let layout =
            Layout::horizontal([cons_percentage(30), cons_percentage(70)]).split(main_layout);

        extract!(op_layout, resp_layout <- layout[0..2]);

        let layout = Layout::vertical([cons_percentage(70), cons_percentage(30)]).split(op_layout);

        extract!(task_layout, input_layout <- layout[0..2]);

        let layout = Layout::vertical([cons_min(3), cons_length(3)]).split(input_layout);

        extract!(server_layout, text_layout <- layout[0..2]);

        frame.render_widget(
            Paragraph::new("Httping")
                .centered()
                .block(Block::bordered()),
            title_layout,
        );

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

        if !self.task_list.is_empty() {
            let selected = self.task_index.selected().unwrap_or_default();
            let task = &self.task_list[selected];
            let respone_list = task.respone();

            debug!("task list count = {}", self.task_list.len());
            if !respone_list.is_empty() {
                let default_header = ["地址", "IP", "状态", "总耗时", "重定向", "重定向耗时"];

                match self.display_style {
                    DisplayStyle::Table => {
                        let mut header = default_header.map(String::from).to_vec();
                        let widths: Vec<Constraint> = vec![];

                        header.extend(respone_list[0].other_name_list().iter().map(String::from));
                        let rows: Vec<_> = respone_list
                            .iter()
                            .map(|respone| {
                                let mut rows = vec![
                                    Text::from(respone.loc()),
                                    Text::from(respone.ip()),
                                    Text::from(respone.status().to_string()),
                                    Text::from(respone.total_cost()),
                                    Text::from(respone.redirect().to_string()),
                                    Text::from(respone.redirect_cost()),
                                ];
                                rows.extend(
                                    respone.other_cost_list().iter().cloned().map(Text::from),
                                );
                                Row::new(rows)
                            })
                            .collect();

                        let table = Table::new(rows, widths)
                            .column_spacing(2)
                            .header(Row::new(header.into_iter().map(|v| Text::from(v).bold())))
                            .highlight_style(Style::new().reversed())
                            .block(
                                Block::bordered()
                                    .title("Respone")
                                    .title_alignment(Alignment::Center),
                            );

                        frame.render_stateful_widget(table, resp_layout, &mut self.resp_index);
                    }
                    DisplayStyle::Total => todo!(),
                    DisplayStyle::Chart(_) => todo!(),
                }
            }
        } else {
            frame.render_widget(
                Paragraph::new("").block(
                    Block::bordered()
                        .title("Respone")
                        .title_alignment(Alignment::Center),
                ),
                resp_layout,
            );
        }

        let mut status: Vec<Span> = vec![];

        let task_count = self.task_list.len();
        let task_complete = self.task_list.iter().filter(|task| task.ending()).count();

        status.push(Span::from(format!("Task {}/{}", task_complete, task_count)));

        if let Some(selected) = self.task_index.selected() {
            let task = &self.task_list[selected];
            let resp = task.respone();
            let success = resp.iter().filter(|v| v.status() == 200).count();

            if success > 0 {
                status.push(Span::from(" | "));
                status.push(Span::from(format!("Respone {}/{}", success, resp.len())));
            }
        }

        frame.render_widget(
            Paragraph::new(Line::from(status)).block(Block::bordered()),
            status_layout,
        );

        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::from("← → Server"),
                Span::from(" | "),
                Span::from("↑ ↓ Task"),
                Span::from(" | "),
                Span::from("⇞ ⇟ Respone"),
                Span::from(" | "),
                Span::from("E(edit mode)"),
            ]))
            .block(Block::bordered()),
            help_layout,
        );
    }

    pub fn update(&mut self, event: Event) -> color_eyre::Result<bool> {
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
                            self.resp_index = TableState::default();
                        }
                        KeyCode::Up => {
                            self.task_index.select_previous();
                            self.resp_index = TableState::default();
                        }
                        KeyCode::Left => {
                            self.server_index.select_previous();
                        }
                        KeyCode::Right => {
                            self.server_index.select_next();
                        }
                        KeyCode::PageUp => {
                            if self.resp_index.offset() > 5 {
                                *self.resp_index.offset_mut() = self.resp_index.offset() - 5;
                            } else {
                                *self.resp_index.offset_mut() = 0;
                            }
                        }
                        KeyCode::PageDown => {
                            if let Some(selected) = self.task_index.selected() {
                                let resp_len = self.task_list[selected].respone().len();

                                if self.resp_index.offset() + 5 < resp_len {
                                    *self.resp_index.offset_mut() = self.resp_index.offset() + 5;
                                }
                            }
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

    pub fn handler<B: Write>(&mut self, _ui: &mut Ui<B>) -> color_eyre::Result<()> {
        for task in self.task_list.iter_mut() {
            task.recv_respone();
        }
        Ok(())
    }
}
