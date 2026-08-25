#![windows_subsystem = "windows"]

use fltk::{
  app, frame::Frame, input::Input, window::Window, button::Button,
  enums::{Align, Color, Event, Font, FrameType, Key, CallbackTrigger},
  table::{TableRow, TableContext}, draw, image::SvgImage, prelude::*
};
use std::{array::from_fn, f64};
use regex::Regex;

const PARTS: [&str; 7] = ["Head", "Chest", "Belly", "Arms", "Forearms", "Thighs", "Legs"];

const PAD_A: i32 = 10; const INP_X: i32 = 255; const RES_X: i32 = 375;
const LBL_W: i32 = 235; const WIDTH: i32 = 100; const HEIGHT: i32 = 30;
const WIN_W: i32 = 365; const WIN_H: i32 = 320; const ROW_H: i32 = 25;

fn validate_input(input: &mut Input, re: Regex) {
  let mut last_valid = input.value();
  input.set_trigger(CallbackTrigger::Changed);
  input.set_callback(move |i| {
    let current = i.value();
    if current.is_empty() || re.is_match(&current) {
      last_valid = current;
    } else {
      let pos = (i.position() - 1).max(0);
      i.set_value(&last_valid);
      drop(i.set_position(pos));
    }
  });
}

fn main() {
  let delta_app = app::App::default().with_scheme(app::Scheme::Gtk);
  app::background(240, 240, 240); app::set_font(Font::Helvetica);

  let mut window = Window::default()
    .with_size(WIN_W, WIN_H).with_label("Delta Force TTK Calculator");

  if let Ok(icon) = SvgImage::from_data(include_str!("../icon.svg")) {
    window.set_icon(Some(icon));
  }

  let mut row_px = PAD_A;
  let damage_re = Regex::new(r"^\d+[.,]?\d* ?(\* ?\d*[.,]?\d*)?$").unwrap();
  let damage_inputs: [Input; PARTS.len()] = from_fn(|i| {
    let part = PARTS[i];

    let mut frame = Frame::default().with_pos(PAD_A, row_px)
      .with_size(LBL_W, ROW_H).with_label(&format!("Damage value for {part}:"));
    frame.set_align(Align::Center | Align::Inside);

    let mut input = Input::default().with_pos(INP_X, row_px).with_size(WIDTH, ROW_H);
    validate_input(&mut input, damage_re.clone());
    row_px += HEIGHT;

    input
  });

  for i in 0..damage_inputs.len() {
    let mut current = damage_inputs[i].clone();

    let up_target = if i > 0 {
      Some(damage_inputs[i - 1].clone())
    } else { None };

    let down_target = if i < damage_inputs.len() - 1 {
      Some(damage_inputs[i + 1].clone())
    } else { None };

    current.handle(move |widget, event| {
      if event == Event::KeyDown {
        match app::event_key() {
          Key::Down => {
            if let Some(mut target) = down_target.clone() {
              target.set_value(&widget.value());
              drop(target.take_focus());
              return true;
            }
          }
          Key::Up => {
            if let Some(mut target) = up_target.clone() {
              target.set_value(&widget.value());
              drop(target.take_focus());
              return true;
            }
          }
          _ => {}
        }
      }
      false
    });
  }

  let mut frame = Frame::default().with_pos(PAD_A, row_px)
    .with_size(LBL_W, ROW_H).with_label("Damage drops (space separated):");
  frame.set_align(Align::Center | Align::Inside);

  let drop_re = Regex::new(r"^((1|0|0?[.,]\d+) )*(1|0|0?[.,]\d*)?$").unwrap();
  let mut drop_input = Input::default().with_pos(INP_X, row_px).with_size(WIDTH, ROW_H);
  validate_input(&mut drop_input, drop_re.clone());
  row_px += HEIGHT;

  let mut frame = Frame::default().with_pos(PAD_A, row_px)
    .with_size(LBL_W, ROW_H).with_label("Weapon firerate (shots per minute):");
  frame.set_align(Align::Center | Align::Inside);

  let rate_re = Regex::new(r"^\d+[.,]?\d*$").unwrap();
  let mut rate_input = Input::default().with_pos(INP_X, row_px).with_size(WIDTH, ROW_H);
  validate_input(&mut rate_input, rate_re.clone());
  row_px += HEIGHT;

  let mut calc_btn = Button::default().with_size(LBL_W + PAD_A + WIDTH, ROW_H)
    .with_pos(PAD_A, row_px).with_label("Calculate TTK for this weapon");
  calc_btn.set_align(Align::Center | Align::Inside);

  calc_btn.handle(move |button, event| {
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

  let mut result_table = TableRow::default().with_pos(RES_X, PAD_A).with_size(0, 0);

  window.end(); window.show();

  let mut first_input = damage_inputs[0].clone();
  let mut window_clone = window.clone();

  calc_btn.set_callback(move |_| {
    let mut damages = [0.0; PARTS.len()];

    for (i, input) in damage_inputs.iter().enumerate() {
      if input.value().is_empty() { return; }

      let mut s = input.value().replace(",", ".");
      s.retain(|c| !c.is_whitespace());
      let s = s.trim_end_matches('.').trim_end_matches('*');

      damages[i] = if let Some((a, b)) = s.split_once('*') {
        a.parse::<f64>().unwrap_or(0.0) * b.parse::<f64>().unwrap_or(0.0)
      } else {
        s.parse::<f64>().unwrap_or(0.0)
      };
    }

    let mut drops: Vec<f64> = drop_input.value().replace(",", ".").split_whitespace()
      .filter(|&s| s != ".").filter_map(|s| s.parse::<f64>().ok()).collect();

    let rate = rate_input.value().replace(",", ".").parse::<f64>().unwrap_or(0.0);

    drops.sort_by(|a, b| b.partial_cmp(a).unwrap());
    if rate <= 0.0 || drops.is_empty() { return; }

    let punish = 60000.0 / rate;
    let mut table_data = Vec::<Vec<String>>::with_capacity(damages.len() + 1);

    let mut header = Vec::<String>::with_capacity(drops.len() + 1);
    header.push("Part / Drop".to_string());
    for drop in &drops { header.push(format!("{drop}x")); }
    table_data.push(header);

    for (i, &damage) in damages.iter().enumerate() {
      let mut row = Vec::<String>::with_capacity(drops.len() + 1);
      row.push(PARTS[i].to_string());
      for &drop in &drops {
        let shots = (100.0 / damage / drop).ceil();
        let ttk = (shots - 1.0) * punish;
        row.push(format!("{shots}t | {ttk:.1}"));
      }
      table_data.push(row);
    }

    let rows = (damages.len() + 1) as i32; let cols = (drops.len() + 1) as i32;

    TableExt::clear(&mut result_table);
    result_table.set_rows(rows);
    result_table.set_cols(cols);
    result_table.set_row_header(false);
    result_table.set_col_header(false);
    result_table.set_row_height_all(HEIGHT);
    result_table.set_col_width_all(WIDTH);

    result_table.draw_cell(move |_, ctx, r, c, x, y, w, h| {
      if ctx == TableContext::Cell {
        draw::push_clip(x, y, w, h);

        let color = if r == 0 || c == 0 { Color::from_hex(0xdddddd) } else { Color::White };
        draw::draw_box(FrameType::ThinUpBox, x, y, w, h, color);

        draw::set_draw_color(Color::Black);
        draw::set_font(Font::Helvetica, 14);
        draw::draw_text2(&table_data[r as usize][c as usize], x, y, w, h, Align::Center);

        draw::pop_clip();
      }
    });

    let (table_width, table_height) = (cols * WIDTH + 4, rows * HEIGHT + 4);
    window_clone.set_size(PAD_A + RES_X + table_width, WIN_H.max(2 * PAD_A + table_height));
    result_table.resize(RES_X, PAD_A, table_width, table_height);

    drop(first_input.take_focus());
  });

  delta_app.run().unwrap();
}