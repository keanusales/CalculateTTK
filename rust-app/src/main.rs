#![windows_subsystem = "windows"]

use fltk::{app, button::Button, enums::{Align, Font},
  frame::Frame, input::Input, window::Window, prelude::*};
use regex::Regex;
use std::f64;

const PARTS: [&str; 7] = ["Head", "Chest", "Abdomen", "Arms", "Forearms", "Thighs", "Legs"];
const FIRERATE_ERROR: &str = "Ensure the firerate value are positive.";
const DROP_ERROR: &str = "Ensure all drops values are between 0 and 1 (0, 1].";
const DAMAGE_ERROR: &str = "Ensure all damages values are settled and positive.";

fn get_ttk_table(damages: &[f64], drops: &[f64], rate: f64) -> Result<String, String> {
  if rate <= 0.0 { return Err(FIRERATE_ERROR.into()); }
  if drops.is_empty() || !drops.iter().all(|&d| d > 0.0 && d <= 1.0) {
    return Err(DROP_ERROR.into());
  }
  if damages.is_empty() || !damages.iter().all(|&d| d > 0.0) {
    return Err(DAMAGE_ERROR.into());
  }

  let punish = 60000.0 / rate;
  let mut rows: Vec<Vec<String>> = Vec::new();

  let mut header = vec!["Part/Drop".to_string()];
  for drop in drops { header.push(format!("{drop}x")); }
  rows.push(header);

  for (i, &damage) in damages.iter().enumerate() {
    let mut row = vec![PARTS[i].to_string()];
    for &drop in drops {
      let ttk = ((100.0 / damage / drop).ceil() - 1.0) * punish;
      row.push(format!("{ttk:.1}"));
    }
    rows.push(row);
  }

  let cols = rows[0].len();
  let mut widths = vec![0; cols];
  for row in &rows {
    for (j, cell) in row.iter().enumerate() {
      widths[j] = widths[j].max(cell.chars().count());
    }
  }

  let (hor, ver, tlhs, trhs, blhs, brhs) = ("═", "║", "╔", "╗", "╚", "╝");
  let (tjoin, bjoin, ljoin, mjoin, rjoin) = ("╦", "╩", "╠", "╬", "╣");

  let line = |lhs: &str, join: &str, rhs: &str| -> String {
    let segments: Vec<String> = widths.iter().map(|w| hor.repeat(*w + 2)).collect();
    format!("{}{}{}", lhs, segments.join(join), rhs)
  };

  let mut output: Vec<String> = vec![line(tlhs, hor, trhs)];

  let total_width: usize = 3 * widths.len() + widths.iter().sum::<usize>() - 1;
  let title_inner = format!(" Punishment is {:.1} ms ", punish);
  let pad_len = total_width.saturating_sub(title_inner.chars().count());
  let half_pad = pad_len / 2;

  let (pad_l, pad_r) = (hor.repeat(half_pad), hor.repeat(pad_len - half_pad));
  let title_line = format!("{ljoin}{pad_l}{title_inner}{pad_r}{rjoin}");

  output.push(title_line);
  output.push(line(ljoin, tjoin, rjoin));

  for (i, row) in rows.iter().enumerate() {
    let padded_cells: Vec<String> = row.iter().enumerate()
      .map(|(j, cell)| format!(" {cell:width$} ", width = widths[j]))
      .collect();
    output.push(format!("{}{}{}", ver, padded_cells.join(ver), ver));

    if i != (rows.len() - 1) { output.push(line(ljoin, mjoin, rjoin)); }
  }

  output.push(line(blhs, bjoin, brhs));
  Ok(output.join("\n"))
}

fn parse_damage(s: &str) -> Result<f64, String> {
  let s = s.replace(" ", "").replace(",", ".");
  if let Some((a, b)) = s.split_once('*') {
    let n1 = a.parse::<f64>().map_err(|_| format!("Invalid value: {a}"))?;
    let n2 = b.parse::<f64>().map_err(|_| format!("Invalid value: {b}"))?;
    Ok(n1 * n2)
  } else {
    s.parse::<f64>().map_err(|_| format!("Invalid value: {s}"))
  }
}

fn apply_validation(input: &mut fltk::input::Input, pattern: &str) {
  let re = Regex::new(&format!("^{pattern}$")).unwrap();

  let mut last_valid = input.value();

  input.set_trigger(fltk::enums::CallbackTrigger::Changed);
  input.set_callback(move |i| {
    let current = i.value();

    if current.is_empty() || re.is_match(&current) {
      last_valid = current;
    } else {
      let pos = i.position() - 1;
      i.set_value(&last_valid);
      let _ = i.set_position(pos);
    }
  });
}

fn main() {
  let delta_app = app::App::default().with_scheme(app::Scheme::Gtk);

  let mut window = Window::default()
    .with_size(360, 350).with_label("Delta Force TTK Calculator");

  let mut damage_inputs = Vec::new();
  let mut row_y = 10;

  for part in PARTS.iter() {
    let mut frame = Frame::default().with_pos(10, row_y)
      .with_size(230, 25).with_label(&format!("Damage value for {part}:"));

    frame.set_align(Align::Center | Align::Inside);

    let mut input = Input::default().with_pos(250, row_y).with_size(100, 25);

    apply_validation(&mut input, r"\d+[.,]?\d* ?(\* ?\d*[.,]?\d*)?");

    damage_inputs.push(input);
    row_y += 30;
  }

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
      if event == fltk::enums::Event::KeyDown {
        match app::event_key() {
          fltk::enums::Key::Down => {
            if let Some(mut target) = down_target.clone() {
              target.set_value(&widget.value());
              let _ = target.take_focus();
              return true;
            }
          }
          fltk::enums::Key::Up => {
            if let Some(mut target) = up_target.clone() {
              target.set_value(&widget.value());
              let _ = target.take_focus();
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
    .with_size(230, 25).with_label("Damage drops (space separated):");
  frame.set_align(Align::Center | Align::Inside);

  let mut drop_input = Input::default().with_pos(250, row_y).with_size(100, 25);
  row_y += 30;

  apply_validation(&mut drop_input, r"1 ?((0?[.,]\d*) ?)*");

  let mut frame = Frame::default().with_pos(10, row_y)
    .with_size(230, 25).with_label("Weapon firerate (shots per minute):");
  frame.set_align(Align::Center | Align::Inside);

  let mut rate_input = Input::default().with_pos(250, row_y).with_size(100, 25);
  row_y += 30;

  apply_validation(&mut rate_input, r"\d+[.,]?\d*");

  let mut calc_btn = Button::default().with_pos(10, row_y)
    .with_size(340, 30).with_label("Calculate TTK for this weapon");

  calc_btn.handle(|button, event| {
    if event == fltk::enums::Event::KeyDown {
      match app::event_key() {
        fltk::enums::Key::KPEnter | fltk::enums::Key::Enter => {
          button.do_callback();
          return true;
        }
        _ => {}
      }
    }
    false
  });

  let mut result_label = Frame::default().with_pos(370, 10).with_size(0, 0);

  result_label.set_label_font(Font::Courier);
  result_label.set_align(Align::Center | Align::Inside);

  window.end(); window.show();

  let mut first_input = damage_inputs[0].clone();
  let mut window_clone = window.clone();

  calc_btn.set_callback(move |_| {
    let process = || -> Result<String, String> {
      let rate: f64 = rate_input.value().parse().map_err(|_| FIRERATE_ERROR.to_string())?;

      let mut damages = Vec::new();
      for input in &damage_inputs {
        if input.value().trim().is_empty() { return Err(DAMAGE_ERROR.to_string()); }
        damages.push(parse_damage(&input.value())?);
      }

      let mut drops: Vec<f64> = drop_input.value().replace(",", ".").split_whitespace()
        .map(|s| s.parse::<f64>().map_err(|_| DROP_ERROR.to_string()))
        .collect::<Result<Vec<_>, _>>()?;

      if drops.is_empty() { drops = vec![1.0]; }
      drops.sort_by(|a, b| b.partial_cmp(a).unwrap());

      get_ttk_table(&damages, &drops, rate)
    };

    match process() {
      Ok(result_table) => result_label.set_label(&result_table),
      Err(error) => result_label.set_label(&format!("An internal error occurred:\n{error}"))
    }

    let (text_w, text_h) = result_label.measure_label();
    result_label.resize(370, 10, text_w, text_h);

    let (new_width, new_height) = (370 + text_w + 20, 350.max(text_h + 20));
    window_clone.set_size(new_width, new_height);

    let _ = first_input.take_focus();
  });

  delta_app.run().unwrap();
}