use nu_ansi_term::{Color, Style};
use reedline::ReedlineMenu::EngineCompleter;
use reedline::{
    default_emacs_keybindings, DefaultCompleter, DefaultHinter, DefaultPrompt, DescriptionMode,
    EditCommand, Emacs, ExampleHighlighter, IdeMenu, KeyCode, KeyModifiers, MenuBuilder, Prompt,
    Reedline, ReedlineEvent,
};

#[derive(Default)]
pub struct CustomPrompt(DefaultPrompt);

impl Prompt for CustomPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<str> {
        "lisp".into()
    }
    fn render_prompt_right(&self) -> std::borrow::Cow<str> {
        self.0.render_prompt_right()
    }
    fn render_prompt_indicator(
        &self,
        prompt_mode: reedline::PromptEditMode,
    ) -> std::borrow::Cow<str> {
        self.0.render_prompt_indicator(prompt_mode)
    }
    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
        self.0.render_prompt_multiline_indicator()
    }
    fn render_prompt_history_search_indicator(
        &self,
        history_search: reedline::PromptHistorySearch,
    ) -> std::borrow::Cow<str> {
        self.0
            .render_prompt_history_search_indicator(history_search)
    }
}

pub struct LineEditorBuilder(pub Reedline);

fn commands() -> Vec<String> {
    let commands = ["+", "-", "*", "/", "(", ")"];
    commands.map(String::from).into_iter().collect()
}

impl LineEditorBuilder {
    pub fn build(self) -> Reedline {
        self.0
    }

    pub fn with_complete(self) -> Self {
        let min_completion_width: u16 = 0;
        let max_completion_width: u16 = 50;
        let max_completion_height: u16 = u16::MAX;
        let padding: u16 = 0;
        let border: bool = true;
        let cursor_offset: i16 = -2;
        let description_mode: DescriptionMode = DescriptionMode::PreferRight;
        let min_description_width: u16 = 0;
        let max_description_width: u16 = 50;
        let description_offset: u16 = 1;
        let correct_cursor_pos: bool = true;

        let commands = commands();
        let completer = Box::new(DefaultCompleter::new_with_wordlen(commands, 0));

        let mut ide_menu = IdeMenu::default()
            .with_name("completion_menu")
            .with_min_completion_width(min_completion_width)
            .with_max_completion_width(max_completion_width)
            .with_max_completion_height(max_completion_height)
            .with_padding(padding)
            .with_cursor_offset(cursor_offset)
            .with_description_mode(description_mode)
            .with_min_description_width(min_description_width)
            .with_max_description_width(max_description_width)
            .with_description_offset(description_offset)
            .with_correct_cursor_pos(correct_cursor_pos);

        if border {
            ide_menu = ide_menu.with_border('┐', '┌', '┘', '└', '─', '│');
        }

        let completion_menu = Box::new(ide_menu);

        let l = self
            .0
            .with_completer(completer)
            .with_menu(EngineCompleter(completion_menu));

        Self(l)
    }

    pub fn with_edit_mode(self) -> Self {
        let mut keybindings = default_emacs_keybindings();
        keybindings.add_binding(
            KeyModifiers::NONE,
            KeyCode::Tab,
            ReedlineEvent::UntilFound(vec![
                ReedlineEvent::Menu("completion_menu".to_string()),
                ReedlineEvent::MenuNext,
            ]),
        );
        keybindings.add_binding(
            KeyModifiers::SHIFT,
            KeyCode::BackTab,
            ReedlineEvent::UntilFound(vec![
                ReedlineEvent::Menu("completion_menu".to_string()),
                ReedlineEvent::MenuPrevious,
            ]),
        );
        keybindings.add_binding(
            KeyModifiers::ALT,
            KeyCode::Enter,
            ReedlineEvent::Edit(vec![EditCommand::InsertNewline]),
        );
        let edit_mode = Box::new(Emacs::new(keybindings));

        let l = self.0.with_edit_mode(edit_mode);
        Self(l)
    }

    pub fn with_highlight(self) -> Self {
        let commands = commands();
        let l = self
            .0
            .with_highlighter(Box::new(ExampleHighlighter::new(commands)));
        Self(l)
    }

    pub fn with_hinter(self) -> Self {
        let l = self.0.with_hinter(Box::new(
            DefaultHinter::default().with_style(Style::new().italic().fg(Color::DarkGray)),
        ));

        Self(l)
    }
}
