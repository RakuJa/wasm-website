use crate::models::app::App;
use ratzilla::WebRenderer;
use ratzilla::event::KeyCode;
use ratzilla::ratatui::Frame;
use ratzilla::ratatui::layout::Flex;
use ratzilla::ratatui::prelude::Stylize;
use ratzilla::ratatui::prelude::*;
use ratzilla::ratatui::widgets::Clear;
use ratzilla::ratatui::{
    layout::{Constraint, Layout},
    style::{
        Color, Modifier, Style,
        palette::tailwind::{AMBER, CYAN, FUCHSIA, GRAY, GREEN, PINK, SLATE},
    },
    text::Line,
    widgets::Widget,
};
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::io;
use std::rc::Rc;
use tachyonfx::fx::RepeatMode;
use tachyonfx::{
    CenteredShrink, Duration, Effect, EffectRenderer, EffectTimer, Interpolation, Motion, Shader,
    fx,
};
use wasm_bindgen::JsValue;
use web_sys::console;
use website::backend::{BackendType, MultiBackendBuilder};

mod models;

const TODO_HEADER_STYLE: Style = Style::new().fg(FUCHSIA.c200).bg(GRAY.c800);
const NORMAL_BG: Color = SLATE.c950;
const SELECTED_STYLE: Style = Style::new()
    .bg(CYAN.c900)
    .add_modifier(Modifier::CROSSED_OUT);
const TEXT_FG_COLOR: Color = GREEN.c300;
const TEXT_DATA_COLOR: Color = AMBER.c300;
const COMPLETED_TEXT_FG_COLOR: Color = PINK.c950;

struct State {
    scene: SceneEnum,
    intro_effect: Effect,
    menu_effect: Effect,
    app: App,
}

#[derive(PartialEq)]
enum SceneEnum {
    Intro,
    List,
}

fn main() -> io::Result<()> {
    let app_state = Rc::new(RefCell::new(State::default()));
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let terminal = MultiBackendBuilder::with_fallback(BackendType::Canvas).build_terminal()?;
    terminal.on_key_event({
        let app_state_cloned = app_state.clone();
        move |event| {
            let mut app_state = app_state_cloned.borrow_mut();
            match event.code {
                KeyCode::Right => {
                    app_state.on_right();
                }
                KeyCode::Left => {
                    app_state.on_left();
                }
                KeyCode::Up => {
                    app_state.on_up();
                }
                KeyCode::Down => {
                    app_state.on_down();
                }
                KeyCode::Enter => {
                    if event.ctrl {
                        app_state.open_link();
                    } else {
                        app_state.on_enter();
                    }
                }
                KeyCode::Char(c) => app_state.on_key(c),
                _ => {}
            }
        }
    });
    terminal.draw_web(move |f| ui(f, &mut app_state.borrow_mut()));
    Ok(())
}

impl Default for State {
    fn default() -> Self {
        Self {
            scene: SceneEnum::Intro,
            intro_effect: fx::sequence(&[
                fx::ping_pong(fx::sweep_in(
                    Motion::LeftToRight,
                    10,
                    0,
                    Color::Magenta,
                    EffectTimer::from_ms(3000, Interpolation::QuadIn),
                )),
                fx::coalesce((3000, Interpolation::SineOut)),
                fx::sleep(20000),
                fx::repeat(
                    fx::sequence(&[
                        fx::sleep(10000),
                        fx::parallel(&[
                            fx::dissolve((10000, Interpolation::BounceOut)),
                            fx::hsl_shift(
                                Some([120.0, 25.0, 25.0]),
                                None,
                                (10000, Interpolation::Linear),
                            ),
                        ]),
                        fx::parallel(&[
                            fx::dissolve((10000, Interpolation::BounceOut)),
                            fx::coalesce((20000, Interpolation::BounceOut)),
                            fx::hsl_shift(
                                Some([120.0, 25.0, 25.0]),
                                None,
                                (20000, Interpolation::Linear),
                            ),
                        ]),
                        fx::sleep(40000),
                    ]),
                    RepeatMode::Forever,
                ),
            ]),
            menu_effect: fx::sequence(&[
                fx::coalesce((3000, Interpolation::SineOut)),
                fx::sleep(1000),
            ]),
            app: App::default(),
        }
    }
}

fn ui(f: &mut Frame<'_>, state: &mut State) {
    render_intro(f, state);
    let value: JsValue = state.intro_effect.running().into();
    console::log_2(&"Is intro running?>".into(), &value);
    if !state.intro_effect.running() || state.scene == SceneEnum::List {
        render_menu(f, state);
    } else {
        render_intro(f, state);
    }
}

impl State {
    pub fn on_down(&mut self) {
        match self.scene {
            SceneEnum::Intro => self.scene = SceneEnum::List,
            SceneEnum::List => self.app.on_down(),
        }
    }

    pub fn on_up(&mut self) {
        match self.scene {
            SceneEnum::Intro => self.scene = SceneEnum::List,
            SceneEnum::List => self.app.on_up(),
        }
    }

    pub fn on_right(&mut self) {
        match self.scene {
            SceneEnum::Intro => self.scene = SceneEnum::List,
            SceneEnum::List => self.app.on_right(),
        }
    }

    pub fn on_left(&mut self) {
        match self.scene {
            SceneEnum::Intro => self.scene = SceneEnum::List,
            SceneEnum::List => self.app.on_left(),
        }
    }

    pub fn on_enter(&mut self) {
        match self.scene {
            SceneEnum::Intro => self.scene = SceneEnum::List,
            SceneEnum::List => self.app.on_enter(),
        }
    }

    pub fn open_link(&mut self) {
        match self.scene {
            SceneEnum::Intro => self.scene = SceneEnum::List,
            SceneEnum::List => self.app.open_link(),
        }
    }
    pub fn on_key(&mut self, c: char) {
        match self.scene {
            SceneEnum::Intro => self.scene = SceneEnum::List,
            SceneEnum::List => self.app.on_key(c),
        }
    }
}

fn render_intro(f: &mut Frame<'_>, state: &mut State) {
    Clear.render(f.area(), f.buffer_mut());
    let area = f.area().inner_centered(43, 3);
    let main_text = Text::from(vec![
        Line::from("| R A K U J A |").bold(),
        Line::from("Daniele Giachetto").italic(),
        Line::from("https://danielegiachetto.com").bold(),
    ]);
    let area_below = Rect {
        x: area.x,
        y: area.y + area.height + 3,
        width: area.width,
        height: 2,
    };
    let secondary_text = Text::from(vec![
        Line::from(".. PRESS ANY (KEYBOARD) KEY TO START ..").italic(),
        Line::from(" ..this site is NOT mobile friendly.."),
    ]);
    f.render_widget(main_text.light_red().centered(), area);
    f.render_widget(secondary_text.light_magenta().centered(), area_below);
    f.render_effect(&mut state.intro_effect, area, Duration::from_millis(30));
}

fn render_menu(f: &mut Frame<'_>, state: &mut State) {
    state.scene = SceneEnum::List;
    Clear.render(f.area(), f.buffer_mut());

    let vertical = Layout::vertical([Constraint::Percentage(90)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]).flex(Flex::Center);
    let [area] = vertical.areas(f.area());
    let [area] = horizontal.areas(area);

    f.render_widget(&mut state.app, area);
    f.render_effect(&mut state.menu_effect, area, Duration::from_millis(100));
}
