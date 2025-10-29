from tkinter.ttk import Frame, Label, Entry, Button
from tkinter import StringVar, Tk
from itertools import pairwise
from re import fullmatch
from math import ceil

def bodyparts() -> tuple[str, str, str, str, str, str, str]:
  return ("Head", "Chest", "Belly", "Arms", "Forearms", "Thighs", "Legs")

def get_ttk_table(damages: list[float], drops: list[float], rate: float) -> str:
  if not (damages and all(damage > 0 for damage in damages)):
    raise ValueError("Ensure all damages values are specified and positive.")
  if not (len(drops) > 1 and all(0 < drop <= 1 for drop in drops)):
    raise ValueError("Ensure all drops values are between 0 and 1 (0, 1].")
  if rate <= 0: raise ValueError("Ensure the firerate value are positive.")

  hor, ver, tleft, tright, bleft, bright = "═", "║", "╔", "╗", "╚", "╝"
  tjoin, bjoin, ljoin, rjoin, mjoin = "╦", "╩", "╠", "╣", "╬"

  def calc_ttk(punish: float, damage: float, drop: float) -> str:
    return f"{(punish * (ceil(100 / damage / drop) - 1)):.1f}"

  punish, parts = (60000 / rate), bodyparts()
  rows = ([["Part/Drop"] + [f"{drop}x" for drop in drops]] +
    [[part] + [calc_ttk(punish, damage, drop) for drop in drops]
      for part, damage in zip(parts, damages, strict = True)])

  widths = [max(len(val) for val in col) for col in zip(*rows)]
  title = (ljoin + f" Punishment is {punish:.1f} ms "
    .center(3 * len(widths) + sum(widths) - 1, hor) + rjoin)

  def line(left: str, join: str, right: str) -> str:
    return left + join.join(hor * (val + 2) for val in widths) + right

  rows = (f"\n{line(ljoin, mjoin, rjoin)}\n"
    .join(f"{ver} {f" {ver} ".join(cell.ljust(val) for cell, val
      in zip(row, widths, strict = True))} {ver}" for row in rows))

  return "\n".join((line(tleft, hor, tright), title,
    line(ljoin, tjoin, rjoin), rows, line(bleft, bjoin, bright)))

def main_interface(root: Tk) -> None:
  root.title("Delta Force TTK Calculator")
  root.resizable(False, False)

  parts = bodyparts()
  frame1 = Frame(root, padding = "5 5 5 5")
  frame1.grid(row = 0, column = 0)
  frame2 = Frame(root, padding = "0 5 5 5")
  frame2.grid(row = 0, column = 1)

  def valid_checker(pattern: str):
    def valid_checker_core(text: str):
      return not (text and fullmatch(pattern, text) is None)
    return (root.register(valid_checker_core), "%P")

  cmd1 = valid_checker(r"\d+[.,]?\d*\s?(\*\s?\d*[.,]?\d*)?")
  damages_entry = [Entry(frame1, width = 15,
    validate = "key", validatecommand = cmd1) for e in parts]
  for row, row_text in enumerate(parts):
    row_text = f"Damage value for {row_text}:"
    Label(frame1, text = row_text).grid(row = row, column = 0)
    damages_entry[row].grid(row = row, column = 1)

  row = len(parts)
  cmd2 = valid_checker(r"1(\s+(0?[.,]?\d*)?)*")
  drop_text = "Damage drops (space separated):"
  Label(frame1, text = drop_text).grid(row = row, column = 0)
  drop_entry = Entry(frame1, width = 15,
    validate = "key", validatecommand = cmd2)
  drop_entry.grid(row = row, column = 1)

  row += 1
  cmd3 = valid_checker(r"\d*\.?\d*")
  rate_text = "Weapon firerate (shots per minute):"
  Label(frame1, text = rate_text).grid(row = row, column = 0)
  rate_entry = Entry(frame1, width = 15,
    validate = "key", validatecommand = cmd3)
  rate_entry.grid(row = row, column = 1)

  result_label = Label(frame2, font = ("Courier New", 10),
    textvariable = (result_text := StringVar()))

  def parse_damage(value: str) -> float:
    stripped_value = "".join(value.split())
    pattern = r"(\d+(?:\.\d+)?)\s?\*\s?(\d+(?:\.\d+)?)"
    match = fullmatch(pattern, stripped_value)
    if not match: return float(stripped_value)
    first_value, second_value = match.groups()
    return float(first_value) * float(second_value)

  def calculate() -> None:
    try:
      rate = float(rate_entry.get())
      damages = [parse_damage(e.get()) for e in damages_entry]
      drops = sorted((float(e.replace(",", "."))
        for e in drop_entry.get().split()), reverse = True)
      result_text.set(get_ttk_table(damages, drops, rate))
    except Exception as e:
      result_text.set(f"An internal error occurred:\n{e!s}")
    result_label.pack(padx = 5, pady = 5)

  def focus_next(widget: Entry | Button) -> str:
    if elem := widget.tk_focusNext(): elem.focus_set()
    if isinstance(elem, Entry): elem.select_range(0, "end")
    if isinstance(widget, Button): widget.invoke()
    return "break"

  def copy_text(entry1: Entry, entry2: Entry) -> str:
    entry2.delete(0, "end")
    entry2.insert(0, entry1.get())
    entry2.focus_set()
    return "break"

  damages_entry[0].focus_set()
  for entry in damages_entry + [drop_entry, rate_entry]:
    entry.bind("<Return>", lambda x: focus_next(x.widget))

  for entry1, entry2 in pairwise(damages_entry):
    entry1.bind("<Down>", lambda x, n = entry2: copy_text(x.widget, n))
    entry2.bind("<Up>", lambda x, n = entry1: copy_text(x.widget, n))

  row += 1
  calcbtn = Button(frame1, text = "Calculate", command = calculate)
  calcbtn.grid(row = row, column = 0, columnspan = 2)
  calcbtn.bind("<Return>", lambda x: focus_next(x.widget))

  for child in frame1.winfo_children(): child.grid(padx = 5, pady = 5)

  root.bind("<Escape>", lambda x: root.destroy())
  root.mainloop()

if __name__ == "__main__": main_interface(Tk(sync = True))