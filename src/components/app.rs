use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, List, ListItem, Paragraph},
};

use super::tui;

fn main() -> Result<()> {

    let handle_ui = thread::spawn(|| {   

        components::app::main_chat();
        
    });

    let mut terminal = tui::Tui::new()?;
    let mut app = App::default();
    app.run(&mut terminal)?;
    Ok(())
}

use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};

//sobe

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    input: String,
    character_index: usize,
    input_mode: InputMode,
    messages: Vec<String>,
}

impl App {
    const fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            character_index: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Retorna o index do byte baseado na posição do caractere.
    ///
    /// Como cada caractere em uma string pode conter vários bytes, é necessário calcular
    /// o índice de bytes com base no índice do caractere.
    fn byte_index(&mut self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }


    // Backspace feito na mão
    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // O método "remove" não é usado no texto salvo para deletar o caracter selecionado.
            // Motivo: usar remove em String funciona em bytes em vez de caracteres.
            // Usar remove exigiria cuidado especial devido aos limites do caractere.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
       
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }
}

pub fn main_chat() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Cria o APP e roda ele
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restaura o terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

// Loop da aplicação
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => app.submit_message(),
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                InputMode::Editing => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Percentage(85),
        Constraint::Percentage(05),
        Constraint::Percentage(10),
        // Constraint::Min(1),
    ]);

    let [messages_area, help_area, input_area] = vertical.areas(f.size());   

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                "Pressione ".into(),
                "q".bold(),
                " para sair, ".into(),
                "e".bold(),
                " para entrar no modo de edição.".bold(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                "Pressione ".into(),
                "Esc".bold(),
                " para sair do modo de edição, ".into(),
                "Enter".bold(),
                " para enviar menssagem".into(),
            ],
            Style::default(),
        ),
    };
    let text = Text::from(Line::from(msg)).patch_style(style);

    let help_message = Paragraph::new(text);
        f.render_widget(help_message, help_area);

    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Menssagem"));
    
    f.render_widget(input, input_area);
    
    match app.input_mode {
        InputMode::Normal =>
            // Deixa o cursor invisivel, o frame faz isso por default.
            {}

        InputMode::Editing => {
            // Fazer o cursor ficar visivel enquanto digita
            #[allow(clippy::cast_possible_truncation)]
            f.set_cursor(
                // Renderizar o cursor na posição atuar do input.
                // A posição pode ser controlade pelas setinhas do teclado.
                input_area.x + app.character_index as u16 + 1,
                // Move uma linha pra baixo, caso digite muito.
                input_area.y + 1,
            );
        }
    }

    let user = "Marquinho";
    let user2 = "Joel";

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {

            if i % 2 == 0 {
                let content = Line::from(Span::raw(format!("{user}: {m}")));
                ListItem::new(content).style(Color::White)
            } else {
                let content = Line::from(Span::raw(format!("{user2}: {m}")));
                ListItem::new(content).style(Color::LightBlue)
            }

            
        })
        .collect();

    // fazer logica do usuario aqui
    let messages = List::new(messages).block(Block::bordered().title("Chat"));
        f.render_widget(messages, messages_area); 
}