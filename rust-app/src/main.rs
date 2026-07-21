#![windows_subsystem = "windows"]

use fltk::{app, button::Button, enums::{Align, Font, Event, Key, CallbackTrigger},
  frame::Frame, input::Input, window::Window, image::SvgImage, prelude::*};
use std::{f64, iter::repeat, fmt::Write, array::from_fn};
use regex::Regex;

const PARTS: [&str; 7] = ["Head", "Chest", "Belly", "Arms", "Forearms", "Thighs", "Legs"];
const DROP_ERROR: &str = "Ensure all drops values are between 0 and 1 (0, 1].";
const DAMAGE_ERROR: &str = "Ensure all damages values are set and positive.";
const FIRERATE_ERROR: &str = "Ensure the firerate value is positive.";

fn get_ttk_table(damages: &[f64], drops: &[f64], rate: f64) -> Result<String, String> {
  if rate <= 0.0 { return Err(FIRERATE_ERROR.into()); }
  if drops.is_empty() || !drops.iter().all(|&d| d > 0.0 && d <= 1.0) {
    return Err(DROP_ERROR.into());
  }
  if damages.is_empty() || !damages.iter().all(|&d| d > 0.0) {
    return Err(DAMAGE_ERROR.into());
  }

  let (part_drop, num_cols) = ("Part/Drop", 1 + drops.len());
  let (punish, mut widths) = (60000.0 / rate, vec![0; num_cols]);

  widths[0] = PARTS.iter().map(|p| p.chars().count()).max()
    .unwrap_or(0).max(part_drop.chars().count());

  let mut ttks_cache: Vec<String> = Vec::with_capacity(damages.len() * drops.len());
  let mut drop_cache: Vec<String> = Vec::with_capacity(drops.len());

  for (i, &drop) in (1..).zip(drops.iter()) {
    let drop_str = format!("{drop}x");
    widths[i] = drop_str.chars().count();
    drop_cache.push(drop_str);
  }

  for &damage in damages {
    for (i, &drop) in (1..).zip(drops.iter()) {
      let ttk = ((100.0 / damage / drop).ceil() - 1.0) * punish;
      let ttk_str = format!("{ttk:.1}");
      widths[i] = widths[i].max(ttk_str.chars().count());
      ttks_cache.push(ttk_str);
    }
  }

  let (hor, ver, tlhs, trhs, blhs, brhs) = ("═", "║", "╔", "╗", "╚", "╝");
  let (tjoin, bjoin, ljoin, mjoin, rjoin) = ("╦", "╩", "╠", "╬", "╣");

  let total_width = 3 * num_cols + widths.iter().sum::<usize>() - 1;
  let mut retbuffer = String::with_capacity(50 * total_width);

  let buffer_write = |
    buffer: &mut String, lhs: &str, join: &str, rhs: &str
  | {
    buffer.push_str(lhs);
    let wlen = widths.len() - 1;
    for (i, &width) in widths.iter().enumerate() {
      buffer.extend(repeat(hor).take(width + 2));
      if i < wlen { buffer.push_str(join); }
    }
    buffer.push_str(rhs);
    buffer.push('\n');
  };

  buffer_write(&mut retbuffer, tlhs, hor, trhs);

  let title_inner = format!(" Punishment is {punish:.1} ms ");
  let pad_len = total_width.saturating_sub(title_inner.chars().count());
  let half_pad = pad_len / 2;

  retbuffer.push_str(ljoin);
  retbuffer.extend(repeat(hor).take(half_pad));
  retbuffer.push_str(&title_inner);
  retbuffer.extend(repeat(hor).take(pad_len - half_pad));
  retbuffer.push_str(rjoin);
  retbuffer.push('\n');

  buffer_write(&mut retbuffer, ljoin, tjoin, rjoin);

  write!(retbuffer, "{ver} {part_drop:width$} ", width = widths[0]).ok();
  for (i, drop_str) in (1..).zip(drop_cache.iter()) {
    write!(retbuffer, "{ver} {drop_str:width$} ", width = widths[i]).ok();
  }
  writeln!(retbuffer, "{ver}").ok();

  buffer_write(&mut retbuffer, ljoin, mjoin, rjoin);

  let mut cache_iter = ttks_cache.into_iter();
  let len_rows = damages.len() - 1;

  for i in 0..damages.len() {
    write!(retbuffer, "{ver} {:width$} ", PARTS[i], width = widths[0]).ok();
    for ii in 1..(drops.len() + 1) {
      let ttk_str = cache_iter.next().unwrap();
      write!(retbuffer, "{ver} {ttk_str:width$} ", width = widths[ii]).ok();
    }
    writeln!(retbuffer, "{ver}").ok();
    if i != len_rows {
      buffer_write(&mut retbuffer, ljoin, mjoin, rjoin);
    }
  }

  buffer_write(&mut retbuffer, blhs, bjoin, brhs);
  Ok(retbuffer)
}

fn parse_damage(s: &str) -> Result<f64, String> {
  let mut s = s.replace(",", ".");
  s.retain(|c| !c.is_whitespace());
  if let Some((a, b)) = s.split_once('*') {
    let n1 = a.parse::<f64>().map_err(|_| format!("Invalid value: {a}"))?;
    let n2 = b.parse::<f64>().map_err(|_| format!("Invalid value: {b}"))?;
    Ok(n1 * n2)
  } else {
    s.parse::<f64>().map_err(|_| format!("Invalid value: {s}"))
  }
}

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
  app::background(240, 240, 240);
  app::set_font(Font::Helvetica);

  let mut window = Window::default()
    .with_size(365, 350).with_label("Delta Force TTK Calculator");

  if let Ok(icon) = SvgImage::from_data(include_str!("../icon.svg")) {
    window.set_icon(Some(icon));
  }

  let mut row_y = 10;
  let damage_re = Regex::new(r"^\d+[.,]?\d* ?(\* ?\d*[.,]?\d*)?$").unwrap();
  let damage_inputs: [Input; 7] = from_fn(|i| {
    let part = PARTS[i];

    let mut frame = Frame::default().with_pos(10, row_y)
      .with_size(235, 25).with_label(&format!("Damage value for {part}:"));
    frame.set_align(Align::Center | Align::Inside);

    let mut input = Input::default().with_pos(255, row_y).with_size(100, 25);
    validate_input(&mut input, damage_re.clone());
    row_y += 30; 

    input 
  });

  let num_inputs = damage_inputs.len();
  for i in 0..num_inputs {
    let mut current = damage_inputs[i].clone();

    let up_target = if i > 0 {
      Some(damage_inputs[i - 1].clone())
    } else { None };

    let down_target = if i < num_inputs - 1 {
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

  let mut frame = Frame::default().with_pos(10, row_y)
    .with_size(235, 25).with_label("Damage drops (space separated):");
  frame.set_align(Align::Center | Align::Inside);

  let drop_re = Regex::new(r"^1 ?((0?[.,]\d*) ?)*$").unwrap();
  let mut drop_input = Input::default().with_pos(255, row_y).with_size(100, 25);
  validate_input(&mut drop_input, drop_re.clone());
  row_y += 30;

  let mut frame = Frame::default().with_pos(10, row_y)
    .with_size(235, 25).with_label("Weapon firerate (shots per minute):");
  frame.set_align(Align::Center | Align::Inside);

  let rate_re = Regex::new(r"^\d+[.,]?\d*$").unwrap();
  let mut rate_input = Input::default().with_pos(255, row_y).with_size(100, 25);
  validate_input(&mut rate_input, rate_re.clone());
  row_y += 30;

  let mut calc_btn = Button::default().with_pos(10, row_y)
    .with_size(345, 30).with_label("Calculate TTK for this weapon");

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

  let mut result_label = Frame::default().with_pos(375, 10).with_size(0, 0);
  result_label.set_label_font(Font::Courier);
  result_label.set_align(Align::Center | Align::Inside);

  window.end(); window.show();

  let mut first_input = damage_inputs[0].clone();
  let mut window_clone = window.clone();

  calc_btn.set_callback(move |_| {
    let process = || -> Result<String, String> {
      let rate: f64 = rate_input.value().replace(",", ".")
        .parse::<f64>().map_err(|_| FIRERATE_ERROR.to_string())?;

      let mut damages = [0.0; 7];
      for (i, input) in damage_inputs.iter().enumerate() {
        if input.value().trim().is_empty() { return Err(DAMAGE_ERROR.to_string()); }
        damages[i] = parse_damage(&input.value())?;
      }

      let mut drops: Vec<f64> = drop_input.value().replace(",", ".")
        .split_whitespace().map(|s| s.parse::<f64>().map_err(|_| DROP_ERROR.to_string()))
        .collect::<Result<Vec<f64>, String>>()?;

      drops.sort_by(|a, b| b.partial_cmp(a).unwrap());
      get_ttk_table(&damages, &drops, rate)
    };

    match process() {
      Ok(result_table) => result_label.set_label(&result_table),
      Err(error) => result_label.set_label(&format!("An error occurred:\n{error}"))
    }

    let (text_w, text_h) = result_label.measure_label();
    result_label.resize(375, 10, text_w, text_h);

    let (new_width, new_height) = (375 + text_w + 20, 350.max(text_h + 20));
    window_clone.set_size(new_width, new_height);

    drop(first_input.take_focus());
  });

  delta_app.run().unwrap();
}