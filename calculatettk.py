from tkinter.ttk import Frame, Label, Entry, Button
from tkinter.scrolledtext import ScrolledText
from math import ceil as top
from tkinter import Tk

############################
###    TTK CALCULATOR    ###
############################

bodyparts = ("Head", "Chest", "Belly", "Arms", "Forearms", "Thighs", "Legs")

def get_ttk(damages: list[float], drop: list[float], rate: float) -> list[list[float]]:
  if len(damages) != len(bodyparts): raise ValueError("Damages must contain 7 numbers.")
  if not all(d > 0 for d in damages): raise ValueError("Damages must be positive.")
  if not all(0 < d <= 1 for d in drop): raise ValueError("Drops must be in (0, 1].")
  if not (rate > 0): raise ValueError("Fire rate must be positive.")
  return [[(60000 / rate) * (top(100 / d / n) - 1) for n in drop] for d in damages]

############################
###   TABLE GENERATION   ###
############################

def get_table(damages: list[float], drop: list[float], rate: float, name: str) -> str:
  h, v, tl, tr, bl, br = "═", "║", "╔", "╗", "╚", "╝"
  tj, bj, lj, rj, mj = "╦", "╩", "╠", "╣", "╬"

  title = f"TTK for {name}" if name else "TTK Calculator"
  rows = [["TTK (ms)"] + [f"{d}x" for d in drop]] + [[p] + [f"{v:.1f}" for v in r]
    for p, r in zip(bodyparts, get_ttk(damages, drop, rate))]

  widths = [max(len(v) for v in col) for col in zip(*rows)]

  def line(left: str, join: str, right: str) -> str:
    return left + join.join(h * (w + 2) for w in widths) + right

  middle_line, len_rows = line(lj, mj, rj), (len(rows) - 1)
  return "\n".join([line(tl, h, tr)]
    + [v + title.center(sum(widths) + 3 * len(widths) - 1) + v]
    + [line(lj, tj, rj)]
    + [item for i, row in enumerate(rows) for item in
        ["".join(f"{v} {c.ljust(w)} " for c, w in zip(row, widths)) + v]
        + ([middle_line] if i < len_rows else [])]
    + [line(bl, bj, br)])

############################
###    MAIN INTERFACE    ###
############################

def main_interface(root: Tk) -> None:
  root.title("Delta Force TTK Calculator")
  root.resizable(False, False)

  frame1 = Frame(root, padding = "5 5 5 5")
  frame1.grid(row = 0, column = 0)
  frame2 = Frame(root, padding = "0 5 5 5")
  frame2.grid(row = 0, column = 1)

  damages_entries = [Entry(frame1, width = 10) for e in bodyparts]
  for row, row_text in enumerate(bodyparts):
    row_text = f"Damage value for {row_text}:"
    Label(frame1, text = row_text).grid(row = row, column = 0)
    damages_entries[row].grid(row = row, column = 1)

  row = len(bodyparts)
  drop_text = "Damage drops (space separated):"
  Label(frame1, text = drop_text).grid(row = row, column = 0)
  drop_entry = Entry(frame1, width = 20)
  drop_entry.grid(row = row, column = 1)

  row += 1
  rate_text = "Weapon fire rate (shots per minute):"
  Label(frame1, text = rate_text).grid(row = row, column = 0)
  rate_entry = Entry(frame1, width = 10)
  rate_entry.grid(row = row, column = 1)

  row += 1
  name_text = "Weapon name (optional):"
  Label(frame1, text = name_text).grid(row = row, column = 0)
  name_entry = Entry(frame1, width = 10)
  name_entry.grid(row = row, column = 1)

  result_text = ScrolledText(frame2, width = 55, height = 20)
  result_text.pack(padx = 10, pady = 10)
  result_text.config(state = "disabled")

  for child in frame1.winfo_children():
    child.grid_configure(padx = 5, pady = 5)
  for child in frame2.winfo_children():
    child.grid_configure(padx = 5, pady = 5)

  def show_results(results: str):
    result_text.config(state = "normal")
    result_text.delete("1.0", "end")
    result_text.insert("end", results)
    result_text.config(state = "disabled")

  def calculate():
    try:
      damage_values = [float(e.get()) for e in damages_entries]
      drop_values = [float(e) for e in drop_entry.get().split()]
      rate_value, name_value = float(rate_entry.get()), name_entry.get().strip()
      show_results(get_table(damage_values, drop_values, rate_value, name_value))
    except Exception as e: show_results(f"Error: {e!s}")

  def focus_next(widget: Entry | Button):
    elem = widget.tk_focusNext()
    if elem: elem.focus_set()
    return "break"

  damages_entries[0].focus_set()
  for entry in damages_entries + [drop_entry, rate_entry, name_entry]:
    entry.bind("<Return>", lambda x: focus_next(x.widget))

  calcbtn = Button(root, text = "Calculate", command = calculate)
  calcbtn.grid(row = 1, column = 0, columnspan = 2, pady = (0, 10))
  calcbtn.bind("<Return>", lambda x: (focus_next(x.widget), calculate()))

  root.mainloop()

#############################
###    RUNNING THE APP    ###
#############################

if __name__ == "__main__": main_interface(root = Tk())