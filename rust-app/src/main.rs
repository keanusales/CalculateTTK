#![windows_subsystem = "windows"]

use fltk::{
  app, button::Button, enums::{Align, Font}, frame::Frame, input::Input,
  text::{TextBuffer, TextDisplay}, window::Window, prelude::*,
};
use std::f64;
use regex::Regex;

const PARTS: [&str; 7] = ["Head", "Chest", "Abdomen", "Arms", "Forearms", "Thighs", "Legs"];

fn get_ttk_table(damages: &[f64], drops: &[f64], rate: f64) -> Result<String, String> {
  if rate <= 0.0 { return Err("A taxa de tiro (firerate) deve ser positiva.".into()); }
  if drops.is_empty() || !drops.iter().all(|&d| d > 0.0 && d <= 1.0) {
    return Err("Todos os drops devem estar entre 0 e 1.".into());
  }
  if damages.is_empty() || !damages.iter().all(|&d| d > 0.0) {
    return Err("Todos os danos devem ser positivos.".into());
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

  let mut output: Vec<String> = Vec::new();
  output.push(line(tlhs, hor, trhs));

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

    if i != (rows.len() - 1) {
      output.push(line(ljoin, mjoin, rjoin));
    }
  }

  output.push(line(blhs, bjoin, brhs));
  Ok(output.join("\n"))
}

fn parse_damage(s: &str) -> Result<f64, String> {
  let s = s.replace(" ", "").replace(",", ".");
  if let Some((a, b)) = s.split_once('*') {
    let n1 = a.parse::<f64>().map_err(|_| format!("Valor inválido: {a}"))?;
    let n2 = b.parse::<f64>().map_err(|_| format!("Valor inválido: {b}"))?;
    Ok(n1 * n2)
  } else {
    s.parse::<f64>().map_err(|_| format!("Valor inválido: {s}"))
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
  let app = app::App::default()
    .with_scheme(app::Scheme::Gtk);

  let mut wind = Window::default()
    .with_size(820, 350)
    .with_label("Delta Force TTK Calculator");

  let mut damage_inputs = Vec::new();
  let mut y = 10;

  for part in PARTS.iter() {
    let mut frame = Frame::default()
      .with_pos(10, y)
      .with_size(150, 25)
      .with_label(&format!("Damage for {part}:"));
    frame.set_align(Align::Left | Align::Inside);

    let mut input = Input::default()
      .with_pos(170, y)
      .with_size(100, 25);

    apply_validation(&mut input, r"\d+[.,]?\d* ?(\* ?\d*[.,]?\d*)?");

    damage_inputs.push(input);
    y += 30;
  }

  let num_inputs = damage_inputs.len();
  for i in 0..num_inputs {
    let mut current = damage_inputs[i].clone();

    let up_target = if i > 0 {
      Some(damage_inputs[i - 1].clone())
    } else {
      None
    };
    let down_target = if i < num_inputs - 1 {
      Some(damage_inputs[i + 1].clone())
    } else {
      None
    };

    current.handle(move |w, ev| {
      if ev == fltk::enums::Event::KeyDown {
        match app::event_key() {
          fltk::enums::Key::Down => {
            if let Some(mut target) = down_target.clone() {
              target.set_value(&w.value());
              let _ = target.take_focus();
              return true;
            }
          }
          fltk::enums::Key::Up => {
            if let Some(mut target) = up_target.clone() {
              target.set_value(&w.value());
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

  let mut frame = Frame::default()
    .with_pos(10, y)
    .with_size(150, 25)
    .with_label("Damage drops (spaces):");
  frame.set_align(Align::Left | Align::Inside);

  let mut drop_input = Input::default()
    .with_pos(170, y)
    .with_size(100, 25);
  y += 30;

  apply_validation(&mut drop_input, r"1 ?((0?[.,]\d*) ?)*");

  let mut frame = Frame::default()
    .with_pos(10, y)
    .with_size(150, 25)
    .with_label("Firerate (SPM):");
  frame.set_align(Align::Left | Align::Inside);

  let mut rate_input = Input::default()
    .with_pos(170, y)
    .with_size(100, 25);
  y += 30;

  apply_validation(&mut rate_input, r"\d+\.?\d*");

  let mut calc_btn = Button::default()
    .with_pos(10, y)
    .with_size(260, 35)
    .with_label("Calculate");

  calc_btn.handle(|b, ev| {
    if ev == fltk::enums::Event::KeyDown {
      match app::event_key() {
        fltk::enums::Key::KPEnter | fltk::enums::Key::Enter => {
          b.do_callback();
          return true;
        }
        _ => {}
      }
    }
    false
  });

  let mut out_disp = TextDisplay::default()
    .with_pos(290, 10)
    .with_size(520, 330);

  out_disp.set_text_font(Font::Courier);
  let mut buf = TextBuffer::default();
  out_disp.set_buffer(buf.clone());

  wind.end();
  wind.show();

  let mut first_input = damage_inputs[0].clone();

  calc_btn.set_callback(move |_| {
    let process = || -> Result<String, String> {
      let rate: f64 = rate_input.value().parse().map_err(|_| "Firerate inválido.".to_string())?;

      let mut damages = Vec::new();
      for inp in &damage_inputs {
        if inp.value().trim().is_empty() { return Err("Preencha todos os danos.".into()); }
        damages.push(parse_damage(&inp.value())?);
      }

      let drop_str = drop_input.value().replace(",", ".");
      let mut drops: Vec<f64> = drop_str.split_whitespace()
        .map(|s| s.parse::<f64>().map_err(|_| "Drop inválido.".to_string()))
        .collect::<Result<Vec<_>, _>>()?;

      if drops.is_empty() { drops = vec![1.0]; }
      drops.sort_by(|a, b| b.partial_cmp(a).unwrap());

      get_ttk_table(&damages, &drops, rate)
    };

    match process() {
      Ok(table) => buf.set_text(&table),
      Err(e) => buf.set_text(&format!("Erro:\n{}", e)),
    }

    let _ = first_input.take_focus();
  });

  let _ = damage_inputs[0].take_focus();

  app.run().unwrap();
}