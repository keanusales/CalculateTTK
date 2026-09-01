#![windows_subsystem = "windows"]

use fltk::{
  app, frame::Frame, input::Input, window::Window, button::Button,
  enums::{Align, Color, Event, Font, FrameType, Key, CallbackTrigger},
  table::{TableRow, TableContext}, draw, image::SvgImage, prelude::*
};
use regex::Regex;

const PARTS: [&str; 7] = ["Head", "Chest", "Belly", "Arms", "Forearms", "Thighs", "Legs"];

const PAD_A: i32 = 10; const INP_X: i32 = 255; const RES_X: i32 = 375;
const LBL_W: i32 = 235; const WIDTH: i32 = 100; const HEIGHT: i32 = 30;
const WIN_W: i32 = 365; const WIN_H: i32 = 320; const ROW_H: i32 = 25;

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
      let _ = input.set_position(pos);
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
  let mut damage_inputs = PARTS.map(|part| {
    let mut frame = Frame::default().with_pos(PAD_A, row_px)
      .with_size(LBL_W, ROW_H).with_label(&format!("Damage value for {part}:"));
    frame.set_align(Align::Center | Align::Inside);

    let mut input = Input::default().with_pos(INP_X, row_px).with_size(WIDTH, ROW_H);
    validate_input(&mut input, damage_re.clone());
    row_px += HEIGHT;
    input
  });

  for (i, input) in damage_inputs.iter().enumerate() {
    let mut target_up = if i > 0 {
      Some(damage_inputs[i - 1].clone())
    } else { None };

    let mut target_down = if i + 1 < damage_inputs.len() {
      Some(damage_inputs[i + 1].clone())
    } else { None };

    let mut current = input.clone();
    current.handle(move |current, event| {
      if event == Event::KeyDown {
        match app::event_key() {
          Key::Down if let Some(target) = target_down.as_mut() => {
            target.set_value(&current.value());
            let _ = target.take_focus();
            return true;
          }
          Key::Up if let Some(target) = target_up.as_mut() => {
            target.set_value(&current.value());
            let _ = target.take_focus();
            return true;
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
  validate_input(&mut drop_input, drop_re);
  row_px += HEIGHT;

  let mut frame = Frame::default().with_pos(PAD_A, row_px)
    .with_size(LBL_W, ROW_H).with_label("Firerate (RPM [Interval] [Bursts]):");
  frame.set_align(Align::Center | Align::Inside);

  let rate_re = Regex::new(r"^(\d+[.,]?\d* ?){1,2}\d*$").unwrap();
  let mut rate_input = Input::default().with_pos(INP_X, row_px).with_size(WIDTH, ROW_H);
  validate_input(&mut rate_input, rate_re);
  row_px += HEIGHT;

  let mut calc_btn = Button::default().with_size(LBL_W + PAD_A + WIDTH, ROW_H)
    .with_pos(PAD_A, row_px).with_label("Calculate TTK for this weapon");
  calc_btn.set_align(Align::Center | Align::Inside);

  calc_btn.handle(|button, event| {
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

  calc_btn.set_callback(move |_| {
    let mut damages = [0.0; PARTS.len()];

    for (input, damage) in damage_inputs.iter_mut().zip(&mut damages) {
      let value = input.value();
      if value.is_empty() { let _ = input.take_focus(); return; }

      let s = value.replace(",", ".").replace(" ", "");
      let s = s.trim_end_matches('.').trim_end_matches('*');

      *damage = if let Some((a, b)) = s.split_once('*') {
        a.parse::<f32>().unwrap_or(0.0) * b.parse::<f32>().unwrap_or(0.0)
      } else {
        s.parse::<f32>().unwrap_or(0.0)
      };

      if *damage <= 0.0 { let _ = input.take_focus(); return; }
    }

    let drops: Vec<f32> = drop_input.value().replace(",", ".").split_whitespace()
      .filter(|&s| s != ".").filter_map(|s| s.parse::<f32>().ok()).collect();

    if drops.is_empty() { let _ = drop_input.take_focus(); return; }

    let rates: Vec<f32> = rate_input.value().replace(",", ".").split_whitespace()
      .filter(|&s| s != ".").filter_map(|s| s.parse::<f32>().ok()).collect();

    let (large_punish, small_punish, bursts) = match rates.as_slice() {
      [rate] if *rate > 0.0 => (60000.0 / *rate, 60000.0 / *rate, 1u32),

      [rate, punish, bursts] if (
        *rate > 0.0 && *punish > 0.0 && *bursts > 1.0 && (60000.0 * *bursts / *rate) > *punish
      ) => {
        (*punish, ((60000.0 * *bursts / *rate) - *punish) / (*bursts - 1.0), *bursts as u32)
      }

      _ => { let _ = rate_input.take_focus(); return; }
    };

    let (rows, cols) = (damages.len() + 1, drops.len() + 1);
    let mut table_data = Vec::<String>::with_capacity(rows * cols);

    table_data.push("Part / Drop".to_string());
    for drop in &drops { table_data.push(format!("{drop}x")); }

    for (&part, &damage) in PARTS.iter().zip(&damages) {
      table_data.push(part.to_string());
      for &drop in &drops {
        let intervals = ((100.0 / damage / drop).ceil() as u32).saturating_sub(1);
        let large_burst = intervals / bursts;
        let small_burst = intervals - large_burst;
        let ttk = (large_burst as f32 * large_punish) + (small_burst as f32 * small_punish);
        table_data.push(format!("{}t | {ttk:.1}", intervals + 1));
      }
    }

    TableExt::clear(&mut result_table);
    result_table.set_rows(rows as i32); result_table.set_cols(cols as i32);
    result_table.set_row_header(false); result_table.set_col_header(false);
    result_table.set_row_height_all(HEIGHT); result_table.set_col_width_all(WIDTH);

    result_table.draw_cell(move |_, ctx, r, c, x, y, w, h| {
      if ctx == TableContext::Cell {
        draw::push_clip(x, y, w, h);

        let color = if r == 0 || c == 0 { Color::from_hex(0xdddddd) } else { Color::White };
        draw::draw_box(FrameType::ThinUpBox, x, y, w, h, color);
        draw::set_font(Font::Helvetica, 14); draw::set_draw_color(Color::Black);

        let index = r as usize * cols + c as usize;
        draw::draw_text2(&table_data[index], x, y, w, h, Align::Center);
        draw::pop_clip();
      }
    });

    let (table_width, table_height) = (cols as i32 * WIDTH + 4, rows as i32 * HEIGHT + 4);
    window.set_size(PAD_A + RES_X + table_width, WIN_H.max(2 * PAD_A + table_height));
    result_table.resize(RES_X, PAD_A, table_width, table_height);

    let _ = damage_inputs[0].take_focus();
  });

  delta_app.run().unwrap();
}