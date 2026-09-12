#![windows_subsystem = "windows"]

use fltk::{
  app, frame::Frame, input::Input, window::Window, button::Button,
  enums::{Align, Color, Event, Font, FrameType, Key, CallbackTrigger},
  table::{TableRow, TableContext}, draw, prelude::*
};
use regex_lite::Regex;

const PARTS: [&str; 7] = [
  "Head", "Chest", "Belly", "Arms", "Forearms", "Thighs", "Legs"
];

const PAD_A: i32 = 10;
const ROW_H: i32 = 25;
const LBL_W: i32 = 235;
const WIDTH: i32 = 110;
const HEIGHT: i32 = 30;
const MAX_INPUT_SIZE: i32 = 40;

const BOX_W: i32 = WIDTH + 10;
const INP_X: i32 = 2 * PAD_A + LBL_W;
const BTN_W: i32 = LBL_W + WIDTH + PAD_A;
const WIN_W: i32 = 2 * PAD_A + BTN_W;
const WIN_H: i32 = 10 * HEIGHT + 2 * PAD_A;
const RES_X: i32 = WIN_W + PAD_A;

fn validate_input(input: &mut Input, re: Regex) {
  let mut last_valid = input.value();
  input.set_trigger(CallbackTrigger::Changed);
  input.set_callback(move |input| {
    let current = input.value();
    if current.is_empty() || re.is_match(&current) {
      last_valid = current;
    } else {
      let pos = (input.position() - 1).max(0);
      input.set_value(&last_valid);
      drop(input.set_position(pos));
    }
  });
}

fn parse_sequence(value: &str) -> Vec<f32> {
  value.replace(",", ".").split_whitespace().filter(|&s| s != ".")
    .filter_map(|s| s.parse::<f32>().ok()).collect()
}

fn main() {
  let delta_app = app::App::default().with_scheme(app::Scheme::Gtk);
  app::background(240, 240, 240);
  app::set_font(Font::Helvetica);

  let mut window = Window::default()
    .with_label("Delta Force TTK Calculator")
    .with_size(WIN_W, WIN_H);

  let damage_re = Regex::new(r"^\d+[.,]?\d* ?(\* ?\d*[.,]?\d*)?$").unwrap();
  let drop_re = Regex::new(r"^((1|0|0?[.,]\d+) )*(1|0|0?[.,]\d*)?$").unwrap();
  let rate_re = Regex::new(r"^(\d+[.,]?\d* ?){1,2}\d*$").unwrap();

  let mut row_px = PAD_A;
  let mut damage_inputs = PARTS.map(|part| {
    let mut frame = Frame::default()
      .with_label(&format!("Damage value for {part}:"))
      .with_size(LBL_W, ROW_H).with_pos(PAD_A, row_px);
    frame.set_align(Align::Center | Align::Inside);

    let mut damage_input = Input::default()
      .with_size(WIDTH, ROW_H).with_pos(INP_X, row_px);
    damage_input.set_maximum_size(MAX_INPUT_SIZE);
    validate_input(&mut damage_input, damage_re.clone());
    row_px += HEIGHT;
    damage_input
  });

  for (i, input) in damage_inputs.iter().enumerate() {
    let mut prev = if i > 0 {
      Some(damage_inputs[i - 1].clone())
    } else { None };

    let mut next = if i + 1 < damage_inputs.len() {
      Some(damage_inputs[i + 1].clone())
    } else { None };

    let mut current = input.clone();
    current.handle(move |current, event| {
      if event == Event::KeyDown {
        match app::event_key() {
          Key::Down if let Some(target) = next.as_mut() => {
            target.set_value(&current.value());
            target.do_callback();
            drop(target.take_focus());
            return true;
          }
          Key::Up if let Some(target) = prev.as_mut() => {
            target.set_value(&current.value());
            target.do_callback();
            drop(target.take_focus());
            return true;
          }
          _ => {}
        }
      }
      false
    });
  }

  let mut frame = Frame::default()
    .with_label("Damage drops (space separated):")
    .with_size(LBL_W, ROW_H).with_pos(PAD_A, row_px);
  frame.set_align(Align::Center | Align::Inside);

  let mut drop_input = Input::default()
    .with_size(WIDTH, ROW_H).with_pos(INP_X, row_px);
  drop_input.set_maximum_size(MAX_INPUT_SIZE);
  validate_input(&mut drop_input, drop_re);
  row_px += HEIGHT;

  let mut frame = Frame::default()
    .with_label("Rate of Fire [Interval] [Bursts]:")
    .with_size(LBL_W, ROW_H).with_pos(PAD_A, row_px);
  frame.set_align(Align::Center | Align::Inside);

  let mut rate_input = Input::default()
    .with_size(WIDTH, ROW_H).with_pos(INP_X, row_px);
  rate_input.set_maximum_size(MAX_INPUT_SIZE);
  validate_input(&mut rate_input, rate_re);
  row_px += HEIGHT;

  let mut button = Button::default()
    .with_label("Calculate TTK for this weapon")
    .with_size(BTN_W, ROW_H).with_pos(PAD_A, row_px);
  button.set_align(Align::Center | Align::Inside);

  button.handle(|button, event| {
    if event == Event::KeyDown {
      match app::event_key() {
        Key::KPEnter | Key::Enter => {
          button.do_callback();
          return true;
        }
        _ => {}
      }
    }
    false
  });

  let mut result = TableRow::default()
    .with_size(0, 0).with_pos(RES_X, PAD_A);

  window.set_xclass("delta_ttk");
  window.end(); window.show();

  button.set_callback(move |_| {
    let mut damages = [0.0; PARTS.len()];

    for (input, damage) in damage_inputs.iter_mut().zip(&mut damages) {
      let value = input.value();
      if value.is_empty() { drop(input.take_focus()); return; }

      let s: String = value.chars().filter_map(|c| match c
        { ' ' => None, ',' => Some('.'), _ => Some(c) }).collect();
      let s = s.trim_end_matches(['.', '*']);

      *damage = if let Some((a, b)) = s.split_once('*') {
        a.parse::<f32>().unwrap_or(0.0) * b.parse::<f32>().unwrap_or(0.0)
      } else {
        s.parse::<f32>().unwrap_or(0.0)
      };

      if *damage <= 0.0 { drop(input.take_focus()); return; }
    }

    let drop_value = drop_input.value();
    if drop_value.is_empty() { drop(drop_input.take_focus()); return; }

    let drops = parse_sequence(&drop_value);
    if !drops.iter().all(|&v| v > 0.0) { drop(drop_input.take_focus()); return; }

    let rate_value = rate_input.value();
    if rate_value.is_empty() { drop(rate_input.take_focus()); return; }

    let rates = parse_sequence(&rate_value);
    let (small_punish, large_punish, bursts) = match rates.as_slice() {
      &[rate] if rate > 0.0 => (60000.0 / rate, 60000.0 / rate, 1.0),
      &[rate, punish, bursts] if (
        rate > 0.0 && punish > 0.0 && bursts > 1.0
        && 60000.0 * bursts / rate > punish
      ) => (
        (60000.0 * bursts / rate - punish) / (bursts - 1.0), punish, bursts
      ),
      _ => { drop(rate_input.take_focus()); return; }
    };

    let (rows, cols) = (damages.len() + 1, drops.len() + 1);
    let mut table_data = Vec::<String>::with_capacity(rows * cols);

    table_data.push("Part / Drop".to_string());
    for drop in &drops { table_data.push(format!("{drop}x")); }

    for (&part, &damage) in PARTS.iter().zip(&damages) {
      table_data.push(part.to_string());
      for &drop in &drops {
        let final_damage = damage * drop;
        let intervals = ((100.0 / final_damage).ceil() - 1.0).max(0.0);
        let large_burst = (intervals / bursts).trunc();
        let small_burst = intervals - large_burst;
        let ttk = large_burst * large_punish + small_burst * small_punish;
        table_data.push(format!("{final_damage:.1}d | {ttk:.1}"));
      }
    }

    TableExt::clear(&mut result);
    result.set_rows(rows as i32);
    result.set_cols(cols as i32);
    result.set_row_header(false);
    result.set_col_header(false);
    result.set_row_height_all(HEIGHT);
    result.set_col_width_all(BOX_W);

    result.draw_cell(move |_, ctx, r, c, x, y, w, h| {
      if ctx == TableContext::Cell {
        draw::push_clip(x, y, w, h);

        let color = if r == 0 || c == 0 { Color::Dark1 } else { Color::White };
        draw::draw_box(FrameType::ThinUpBox, x, y, w, h, color);
        draw::set_font(Font::Helvetica, 14);
        draw::set_draw_color(Color::Black);

        let data = &table_data[r as usize * cols + c as usize];
        draw::draw_text2(data, x, y, w, h, Align::Center | Align::Inside);
        draw::pop_clip();
      }
    });

    let (width, height) = (cols as i32 * BOX_W + 4, rows as i32 * HEIGHT + 4);
    window.set_size(PAD_A + RES_X + width, WIN_H.max(2 * PAD_A + height));
    result.resize(RES_X, PAD_A, width, height);

    drop(damage_inputs[0].take_focus());
  });

  delta_app.run().unwrap();
}