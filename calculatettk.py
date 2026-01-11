from tkinter.ttk import Frame, Label, Entry, Button
from tkinter import StringVar, Widget, Tk
from re import compile as re_compile
from itertools import pairwise
from math import ceil

def bodyparts() -> tuple[str, str, str, str, str, str, str]:
  return ("Head", "Chest", "Abdomen", "Arms", "Forearms", "Thighs", "Legs")

def get_ttk_table(damages: list[float], drops: list[float], rate: float) -> str:
  if rate <= 0: raise ValueError("Ensure the firerate value are positive.")
  if not drops or not all(0 < drop <= 1 for drop in drops):
    raise ValueError("Ensure all drops values are between 0 and 1 (0, 1].")
  if not damages or not all(damage > 0 for damage in damages):
    raise ValueError("Ensure all damages values are settled and positive.")

  punish, parts = (60000 / rate), bodyparts()
  hor, ver, tlhs, trhs, blhs, brhs = "═", "║", "╔", "╗", "╚", "╝"
  tjoin, bjoin, ljoin, mjoin, rjoin = "╦", "╩", "╠", "╬", "╣"

  def get_ttk(damage: float, drop: float) -> str:
    return f"{((ceil(100 / damage / drop) - 1) * punish):.1f}"

  rows = [["Part/Drop"] + [f"{drop}x" for drop in drops]]
  rows += [[part] + [get_ttk(damage, drop) for drop in drops]
    for part, damage in zip(parts, damages, strict = True)]

  widths = [max(len(v) for v in c) for c in zip(*rows, strict = True)]

  def line(lhs: str, join: str, rhs: str) -> str:
    return f"{lhs}{join.join(hor * (i + 2) for i in widths)}{rhs}"

  rows = (f"\n{line(ljoin, mjoin, rjoin)}\n"
    .join(f"{ver} {f" {ver} ".join(s.ljust(w) for s, w in
      zip(r, widths, strict = True))} {ver}" for r in rows))

  title = f"{ljoin}{f" Punishment is {punish:.1f} ms "
    .center(3 * len(widths) + sum(widths) - 1, hor)}{rjoin}"

  return "\n".join((line(tlhs, hor, trhs), title,
    line(ljoin, tjoin, rjoin), rows, line(blhs, bjoin, brhs)))

def main_interface(root: Tk) -> None:
  root.title("Delta Force TTK Calculator")
  root.resizable(False, False)

  parts = bodyparts()
  frame1 = Frame(root, padding = "5 5 5 5")
  frame1.grid(row = 0, column = 0)
  frame2 = Frame(root, padding = "0 5 5 5")
  frame2.grid(row = 0, column = 1)

  def valid_checker(pattern: str) -> tuple[str, str]:
    compiled_checker = re_compile(pattern)
    def valid_checker_core(text: str) -> bool:
      fullmatch = compiled_checker.fullmatch(text)
      return not text or fullmatch is not None
    return (root.register(valid_checker_core), "%P")

  checker = valid_checker(r"\d+[.,]?\d* ?(\* ?\d*[.,]?\d*)?")
  damages_entry = [Entry(frame1, width = 15,
    validate = "key", validatecommand = checker) for e in parts]
  for row, row_text in enumerate(parts):
    row_text = f"Damage value for {row_text}:"
    Label(frame1, text = row_text).grid(row = row, column = 0)
    damages_entry[row].grid(row = row, column = 1)

  row = len(parts)
  checker = valid_checker(r"1 ?((0?[.,]\d*) ?)*")
  drop_text = "Damage drops (space separated):"
  Label(frame1, text = drop_text).grid(row = row, column = 0)
  drop_entry = Entry(frame1, width = 15,
    validate = "key", validatecommand = checker)
  drop_entry.grid(row = row, column = 1)

  row += 1
  checker = valid_checker(r"\d+\.?\d*")
  rate_text = "Weapon firerate (shots per minute):"
  Label(frame1, text = rate_text).grid(row = row, column = 0)
  rate_entry = Entry(frame1, width = 15,
    validate = "key", validatecommand = checker)
  rate_entry.grid(row = row, column = 1)

  result_text, font = StringVar(), ("Consolas", 10)
  result_label = Label(frame2, textvariable = result_text,
    font = font, anchor = "center", justify = "center")

  damage_parser = re_compile(r"(\d+(?:\.\d+)?)\*(\d+(?:\.\d+)?)")
  def parse_damage(damage_value: str) -> float:
    stripped_value = "".join(damage_value.split())
    fullmatch = damage_parser.fullmatch(stripped_value)
    if fullmatch is None: return float(stripped_value)
    first_value, second_value = fullmatch.groups()
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
    if next := widget.tk_focusNext(): next.focus_set()
    if isinstance(next, Entry): next.select_range(0, "end")
    if isinstance(widget, Button): widget.invoke()
    return "break"

  def copy_text(entry1: Entry, entry2: Entry) -> str:
    entry2.delete(0, "end")
    entry2.insert(0, entry1.get())
    entry2.focus_set()
    return "break"

  damages_entry[0].focus_set()
  for entry in damages_entry + [drop_entry, rate_entry]:
    entry.bind("<Return>", lambda w: focus_next(w.widget))

  for entry1, entry2 in pairwise(damages_entry):
    entry1.bind("<Down>", lambda w, n = entry2: copy_text(w.widget, n))
    entry2.bind("<Up>", lambda w, n = entry1: copy_text(w.widget, n))

  row += 1
  calcbtn = Button(frame1, text = "Calculate", command = calculate)
  calcbtn.grid(row = row, column = 0, columnspan = 2)
  calcbtn.bind("<Return>", lambda w: focus_next(w.widget))

  for child in frame1.winfo_children():
    if isinstance(child, Widget): child.grid(padx = 5, pady = 5)

  root.bind("<Escape>", lambda w: root.destroy())
  root.mainloop()

if __name__ == "__main__": main_interface(Tk(sync = True))