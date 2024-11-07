use std::collections::{BTreeMap, VecDeque};

use console::{style, Term};
use zellij_tile::prelude::*;

const KEYS_SIZE: usize = 10;
const REMOVE_KEYS_TIMEOUT: Option<f64> = Some(5.);

#[derive(Default)]
struct State {
    keys: VecDeque<KeyWithModifier>,
    term: Option<Term>,
    last_update: Option<std::time::Instant>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[EventType::Key, EventType::Timer]);
        self.term = Some(Term::stdout());
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::Key(key) => {
                self.keys.push_front(key);
                if self.keys.len() > KEYS_SIZE {
                    self.keys.pop_back();
                }
                // if let Some(remove_keys_timeout) = REMOVE_KEYS_TIMEOUT {

                // }
                set_timeout(0.5);
                eprintln!("Key event");
                self.last_update = Some(std::time::Instant::now());
                should_render = true;
            }
            Event::Timer(_t) => {
                eprintln!("Timer event");
                self.keys.pop_front();
                should_render = true;
            }
            _ => (),
        };
        should_render
    }

    fn render(&mut self, rows: usize, _cols: usize) {
        let term = self.term.as_mut().unwrap();

        term.write_line(format!("last update was: {:?}", self.last_update).as_str())
            .unwrap();

        for key in self.keys.iter().take(rows) {
            let s = format!("{}", style(format!("{key}")).white().on_black().bold());
            term.write_line(&s).unwrap();
        }
    }
}
