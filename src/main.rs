use std::collections::{BTreeMap, VecDeque};

use console::{style, Term};
use zellij_tile::prelude::*;

const SIZE: usize = 10;

#[derive(Default)]
struct State {
    keys: VecDeque<KeyWithModifier>,
    term: Option<Term>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[EventType::Key]);
        self.term = Some(Term::stdout());
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::Key(key) => {
                self.keys.push_front(key);
                if self.keys.len() > SIZE {
                    self.keys.pop_back();
                }
                should_render = true;
            }
            _ => (),
        };
        should_render
    }

    fn render(&mut self, rows: usize, _cols: usize) {
        let term = self.term.as_mut().unwrap();

        for key in self.keys.iter().take(rows) {
            let s = format!("{}", style(format!("{key}")).white().on_black().bold());
            term.write_line(&s).unwrap();
        }
    }
}
