#![windows_subsystem = "windows"]

use fltk::{
  app, frame::Frame, input::Input, window::Window, button::Button,
  enums::{Align, Color, Event, Font, FrameType, Key, CallbackTrigger},
  table::{TableRow, TableContext}, draw, prelude::*
};
use regex_lite::Regex;

const PARTS_LEN: usize = 7;
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
const WIN_H: i32 = 12 * HEIGHT + 2 * PAD_A;
const RES_X: i32 = WIN_W + PAD_A;

const GRAPH_HEIGHT: i32 = 300;
const GRAPH_PAD_X: i32 = 55;
const GRAPH_PAD_Y: i32 = 25;
const GRAPH_RIGHT_PAD: i32 = 15;
const GRAPH_UPPER_PAD: i32 = 15;

const LABEL_WIDTH: i32 = 40;
const LABEL_HEIGHT: i32 = 20;
const TTK_LINE_WIDTH: i32 = 2;
const GRID_LINE_WIDTH: i32 = 1;
const GRIDY_STEPS: i32 = 5;

const PARTS: [&str; PARTS_LEN] = [
  "Head", "Chest", "Belly", "Arms", "Forearms", "Thighs", "Legs"
];

const COLORS: [Color; PARTS_LEN] = [
  Color::Red, Color::Green, Color::Blue, Color::Magenta,
  Color::Cyan, Color::Yellow, Color::Black
];

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
  value.replace(",", ".")
    .split_whitespace().filter(|&s| s != ".")
    .filter_map(|s| s.parse::<f32>().ok())
    .collect()
}

macro_rules! invalid_input {
  ($input:expr) => {
    drop($input.take_focus()); return;
  };
}

fn main() {
  let delta_app = app::App::default().with_scheme(app::Scheme::Gtk);
  app::background(240, 240, 240);
  app::set_font(Font::Helvetica);

  let mut window = Window::default()
    .with_label("Delta Force TTK and Ballistics Calculator")
    .with_size(WIN_W, WIN_H);

  let damage_re = Regex::new(r"^\d+[.,]?\d* ?(\* ?\d*[.,]?\d*)?$").unwrap();
  let drop_re = Regex::new(r"^((1|0|0?[.,]\d+) )*(1|0|0?[.,]\d*)?$").unwrap();
  let dist_re = Regex::new(r"^(\d+[.,]?\d* )*(\d+[.,]?\d*)?$").unwrap();
  let rate_re = Regex::new(r"^(\d+[.,]?\d* ?){1,2}\d*$").unwrap();
  let veloc_re = Regex::new(r"^\d+[.,]?\d*$").unwrap();

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
    .with_label("Distances for drops (m):")
    .with_size(LBL_W, ROW_H).with_pos(PAD_A, row_px);
  frame.set_align(Align::Center | Align::Inside);

  let mut dist_input = Input::default()
    .with_size(WIDTH, ROW_H).with_pos(INP_X, row_px);
  dist_input.set_maximum_size(MAX_INPUT_SIZE);
  validate_input(&mut dist_input, dist_re);
  row_px += HEIGHT;

  let mut frame = Frame::default()
    .with_label("Bullet Velocity (m/s):")
    .with_size(LBL_W, ROW_H).with_pos(PAD_A, row_px);
  frame.set_align(Align::Center | Align::Inside);

  let mut veloc_input = Input::default()
    .with_size(WIDTH, ROW_H).with_pos(INP_X, row_px);
  veloc_input.set_maximum_size(MAX_INPUT_SIZE);
  validate_input(&mut veloc_input, veloc_re);
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
    .with_label("Calculate TTK and Plot Graph")
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

  let mut end_graph = Frame::default().with_size(0, 0);
  let mut end_table = TableRow::default().with_size(0, 0);
  end_table.end();

  window.set_xclass("delta_ttk");
  window.end(); window.show();

  button.set_callback(move |_| {
    let mut damages = [0.0; PARTS_LEN];

    for (input, damage) in damage_inputs.iter_mut().zip(&mut damages) {
      let value = input.value();
      if value.is_empty() { invalid_input!(input); }

      let s: String = value.chars().filter_map(|c| match c
        { ' ' => None, ',' => Some('.'), _ => Some(c) }).collect();
      let s = s.trim_end_matches(['.', '*']);

      *damage = if let Some((a, b)) = s.split_once('*') {
        a.parse::<f32>().unwrap_or(0.0) * b.parse::<f32>().unwrap_or(0.0)
      } else {
        s.parse::<f32>().unwrap_or(0.0)
      };

      if *damage <= 0.0 { invalid_input!(input); }
    }

    let drop_value = drop_input.value();
    if drop_value.is_empty() { invalid_input!(drop_input); }

    let drops = parse_sequence(&drop_value);
    if !drops.iter().all(|&v| v > 0.0) { invalid_input!(drop_input); }

    let dist_value = dist_input.value();
    let mut dists = parse_sequence(&dist_value);
    dists.insert(0, 0.0);

    if dists.len() != drops.len() { invalid_input!(dist_input); }

    let velocity = veloc_input.value()
      .replace(",", ".").parse::<f32>().unwrap_or(0.0);
    if velocity <= 0.0 { invalid_input!(veloc_input); }

    let rate_value = rate_input.value();
    if rate_value.is_empty() { invalid_input!(rate_input); }

    let rates = parse_sequence(&rate_value);
    let (small_punish, large_punish, bursts) = match rates.as_slice() {
      &[rate] if rate > 0.0 => (60000.0 / rate, 60000.0 / rate, 1.0),
      &[rate, punish, bursts] if (
        rate > 0.0 && punish > 0.0 && bursts > 1.0
        && 60000.0 * bursts / rate > punish
      ) => (
        (60000.0 * bursts / rate - punish) / (bursts - 1.0), punish, bursts
      ),
      _ => { invalid_input!(rate_input); }
    };

    let (rows, cols) = (PARTS_LEN + 1, drops.len() + 1);
    let mut table_data = Vec::<String>::with_capacity(rows * cols);

    table_data.push("Part / Range".to_string());
    for (pair, &drop) in dists.windows(2).zip(&drops) {
      table_data.push(format!("{}-{}m {}x", pair[0], pair[1], drop));
    }

    let last = drops.len() - 1;
    let last_dist = unsafe { dists.get_unchecked(last) };
    let last_drop = unsafe { drops.get_unchecked(last) };
    table_data.push(format!("{last_dist}+m {last_drop}x"));

    let mut part_ttks = Vec::<f32>::with_capacity(PARTS_LEN * drops.len());

    for (&part, &damage) in PARTS.iter().zip(&damages) {
      table_data.push(part.to_string());
      for (&drop, &dist) in drops.iter().zip(&dists) {
        let final_damage = damage * drop;
        let intervals = ((100.0 / final_damage).ceil() - 1.0).max(0.0);
        let large_burst = (intervals / bursts).trunc();
        let small_burst = intervals - large_burst;

        let base_ttk = large_burst * large_punish + small_burst * small_punish;
        let final_ttk = base_ttk + (dist / velocity) * 1000.0;
        table_data.push(format!("{final_damage:.1}d | {final_ttk:.1}"));
        part_ttks.push(base_ttk);
      }
    }

    TableExt::clear(&mut end_table);
    end_table.set_rows(rows as i32);
    end_table.set_cols(cols as i32);
    end_table.set_row_header(false);
    end_table.set_col_header(false);
    end_table.set_row_height_all(HEIGHT);
    end_table.set_col_width_all(BOX_W);

    end_table.draw_cell(move |_, ctx, r, c, x, y, w, h| {
      if ctx == TableContext::Cell {
        draw::push_clip(x, y, w, h);

        let color = if r == 0 || c == 0 { Color::Dark1 } else { Color::White };
        draw::draw_box(FrameType::ThinUpBox, x, y, w, h, color);
        draw::set_font(Font::Helvetica, 14);
        draw::set_draw_color(Color::Black);

        let index = r as usize * cols + c as usize;
        let data = unsafe { table_data.get_unchecked(index) };
        draw::draw_text2(data, x, y, w, h, Align::Center | Align::Inside);
        draw::pop_clip();
      }
    });

    let table_w = cols as i32 * BOX_W + 4;
    let table_h = rows as i32 * HEIGHT + 4;
    end_table.resize(RES_X, PAD_A, table_w, table_h);

    let graph_iy = PAD_A + table_h + PAD_A;

    let slope = 1000.0 / velocity;
    let max_dist = (last_dist * 1.2).max(100.0);

    let max_ttk = part_ttks.chunks(drops.len())
      .filter_map(|chunk| chunk.last())
      .map(|&clast| slope * max_dist + clast)
      .fold(0.0, f32::max).max(100.0);

    end_graph.draw(move |frame| {
      draw::push_clip(frame.x(), frame.y(), frame.w(), frame.h());
      draw::set_font(Font::Helvetica, 12);

      draw::draw_box(FrameType::FlatBox,
        frame.x(), frame.y(), frame.w(), frame.h(), Color::Black);
      draw::draw_box(FrameType::BorderBox,
        frame.x(), frame.y(), frame.w(), frame.h(), Color::Dark1);

      let graph_w = frame.w() - GRAPH_PAD_X - GRAPH_RIGHT_PAD;
      let graph_h = frame.h() - GRAPH_PAD_Y - GRAPH_UPPER_PAD;
      let graph_x = frame.x() + GRAPH_PAD_X;
      let graph_y = frame.y() + frame.h() - GRAPH_PAD_Y;

      let scale_x = graph_w as f32 / max_dist;
      let scale_y = graph_h as f32 / max_ttk;

      draw::set_line_style(draw::LineStyle::Dot, GRID_LINE_WIDTH);

      for i in 0..=GRIDY_STEPS {
        let value = max_ttk * (i as f32 / GRIDY_STEPS as f32);
        let pixel = graph_y - (value * scale_y) as i32;

        draw::set_draw_color(Color::Dark3);
        draw::draw_line(graph_x, pixel, graph_x + graph_w, pixel);
        let sub_factor = (GRIDY_STEPS - i) * LABEL_HEIGHT / GRIDY_STEPS;

        draw::set_draw_color(Color::Black);
        draw::draw_text2(&format!("{value:.0}ms"),
          frame.x(), pixel - sub_factor, GRAPH_PAD_X - 5, LABEL_HEIGHT,
          Align::Right | Align::Inside);
      }

      let gridx_steps = (cols * 2) as i32;
      for i in 0..=gridx_steps {
        let value = max_dist * (i as f32 / gridx_steps as f32);
        let pixel = graph_x + (value * scale_x) as i32;

        draw::set_draw_color(Color::Dark3);
        draw::draw_line(pixel, graph_y, pixel, graph_y - graph_h);

        draw::set_draw_color(Color::Black);
        draw::draw_text2(&format!("{value:.0}m"),
          pixel - LABEL_WIDTH, graph_y, LABEL_WIDTH, LABEL_HEIGHT,
          Align::Right | Align::Inside);
      }

      for (part, ttks) in part_ttks.chunks(drops.len()).enumerate() {
        if damages[..part].contains(&damages[part]) { continue; }

        draw::set_draw_color(COLORS[part]);
        draw::set_line_style(draw::LineStyle::Solid, TTK_LINE_WIDTH);

        for (i, (&dist_init, &b)) in dists.iter().zip(ttks).enumerate() {
          let &dist_end = dists.get(i + 1).unwrap_or(&max_dist);

          let y_start = slope * dist_init + b;
          let y_end = slope * dist_end + b;

          let px1 = graph_x + (dist_init * scale_x) as i32;
          let py1 = graph_y - (y_start * scale_y) as i32;
          let px2 = graph_x + (dist_end * scale_x) as i32;
          let py2 = graph_y - (y_end * scale_y) as i32;

          draw::draw_line(px1, py1, px2, py2);

          if let Some(&b_next) = ttks.get(i + 1) {
            let y_jump = slope * dist_end + b_next;
            let py3 = graph_y - (y_jump * scale_y) as i32;

            draw::set_line_style(draw::LineStyle::Dot, TTK_LINE_WIDTH);
            draw::draw_line(px2, py2, px2, py3);
            draw::set_line_style(draw::LineStyle::Solid, TTK_LINE_WIDTH);
          }
        }
      }

      draw::pop_clip();
    });

    end_graph.resize(RES_X, graph_iy, table_w, GRAPH_HEIGHT);
    end_graph.redraw();

    let total_h = (graph_iy + GRAPH_HEIGHT + PAD_A).max(WIN_H);
    window.set_size(PAD_A + RES_X + table_w, total_h);

    drop(damage_inputs[0].take_focus());
  });

  delta_app.run().unwrap();
}