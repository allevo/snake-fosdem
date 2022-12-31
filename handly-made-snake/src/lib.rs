mod utils;

use js_sys::{Int32Array, Object};
use snake::{Direction, Game, Snapshot, SNAKE_1, SNAKE_2};
use tracing::{instrument, info, Span};
use tracing_subscriber::{fmt::{time::UtcTime, format::{FmtSpan, Pretty}}, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! _log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, handly-made!");
}

#[wasm_bindgen]
pub struct GameWrapper(Game);

#[wasm_bindgen]
pub struct PointWrapper(usize, usize);

#[wasm_bindgen]
impl GameWrapper {
    pub fn dim(&self) -> Int32Array {
        let dim = self.0.dim();

        let dim = vec![dim.0 as i32, dim.1 as i32];

        Int32Array::from(&dim[..])
    }

    pub fn walls(&self) -> Int32Array {
        let walls: Vec<i32> = self
            .0
            .walls()
            .into_iter()
            .flat_map(|p| vec![p.x as i32, p.y as i32].into_iter())
            .collect();

        Int32Array::from(&walls[..])
    }

    pub fn tick(&mut self, direction: DirectionWrapper) -> SnapshotWrapper {
        info!("direction = {:?}", direction);

        let direction = match direction {
            DirectionWrapper::Up => Direction::Up,
            DirectionWrapper::Down => Direction::Down,
            DirectionWrapper::Left => Direction::Left,
            DirectionWrapper::Right => Direction::Right,
        };

        self.0.tick(direction);

        self.snapshot()
    }

    pub fn snapshot(&self) -> SnapshotWrapper {
        SnapshotWrapper(self.0.last_snapshot())
    }
}

#[wasm_bindgen]
pub struct SnapshotWrapper(Snapshot);

#[wasm_bindgen]
impl SnapshotWrapper {
    pub fn snake(&self) -> Int32Array {
        let s: Vec<_> = self
            .0
            .snake
            .iter()
            .cloned()
            .flat_map(|p| vec![p.x as i32, p.y as i32].into_iter())
            .collect();
        js_sys::Int32Array::from(&s[..])
    }

    pub fn food(&self) -> Int32Array {
        let dim = vec![self.0.food_position.x as i32, self.0.food_position.y as i32];
        Int32Array::from(&dim[..])
    }

    pub fn die_reason(&self) -> JsValue {
        if self.0.on_wall {
            return JsValue::from_str("on wall");
        }

        if self.0.eat_itself {
            return JsValue::from_str("on snake");
        }

        JsValue::NULL
    }

    pub fn score(&self) -> usize {
        self.0.score
    }

    pub fn period_duration_ms(&self) -> usize {
        self.0.period_duration.as_millis() as usize
    }
}
#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(tracing_web::MakeConsoleWriter)
        .with_span_events(FmtSpan::ACTIVE)
        .with_span_events(FmtSpan::CLOSE);
    let perf_layer = tracing_web::performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

#[wasm_bindgen]
pub fn create_game(level_name: JsValue) -> GameWrapper {
    let level_name = level_name.as_string().unwrap();
    let game: Game = match level_name.as_str() {
        "snake1" => SNAKE_1.parse().unwrap(),
        "snake2" => SNAKE_2.parse().unwrap(),
        _ => panic!("Unknown level_name"),
    };

    GameWrapper(game)
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionWrapper {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[wasm_bindgen]
pub fn levels() -> Object {
    let levels = Object::new();

    js_sys::Reflect::set(&levels, &"snake1".into(), &SNAKE_1.into()).unwrap();
    js_sys::Reflect::set(&levels, &"snake2".into(), &SNAKE_2.into()).unwrap();

    levels
}
