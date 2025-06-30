from tkinter.ttk import Frame, Label, Entry, Button
from itertools import pairwise
from tkinter import Text, Tk
from re import fullmatch
from math import ceil

############################
###    TTK CALCULATOR    ###
############################

bodyparts = ("Head", "Chest", "Belly", "Arms", "Forearms", "Thighs", "Legs")

def get_ttk(damages: list[float], drops: list[float], rate: float) -> list[list[float]]:
  if (len(damages) < len(bodyparts) or not all(d > 0 for d in damages)
    or not drops or not all(0 < d <= 1 for d in drops) or not (rate > 0)):
      raise ValueError("Theres an invalid input. Ensure the values are correct.")
  return [[(60000 / rate) * (ceil(100 / d / n) - 1) for n in drops] for d in damages]

############################
###   TABLE GENERATION   ###
############################

def get_table(damages: list[float], drops: list[float], rate: float) -> tuple[str, int]:
  hor, ver, tleft, tright, bleft, bright = "═", "║", "╔", "╗", "╚", "╝"
  tjoin, bjoin, ljoin, rjoin, mjoin = "╦", "╩", "╠", "╣", "╬"

  rows = ([["Part/Drop"] + [f"{drop}x" for drop in drops]]
    + [[part] + [f"{ttk:.1f}" for ttk in ttks] for part, ttks
      in zip(bodyparts, get_ttk(damages, drops, rate))])
  widths = [max(len(value) for value in column) for column in zip(*rows)]

  def line(left: str, join: str, right: str) -> str:
    return left + join.join(hor * (w + 2) for w in widths) + right

  max_width = sum(widths) + 3 * len(widths) + 1
  middle_line, len_rows = line(ljoin, mjoin, rjoin), (len(rows) - 1)

  monted_table = "\n".join([line(tleft, hor, tright)]
    + [ljoin + " Miliseconds TTKs ".center(max_width - 2, hor) + rjoin]
    + [line(ljoin, tjoin, rjoin)]
    + [item for i, row in enumerate(rows) for item in
      ["".join(f"{ver} {c.ljust(w)} " for c, w in zip(row, widths)) + ver]
      + ([middle_line] if i < len_rows else [])]
    + [line(bleft, bjoin, bright)])

  return monted_table, max_width

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

  damages_entry = [Entry(frame1, width = 15) for e in bodyparts]
  for row, row_text in enumerate(bodyparts):
    row_text = f"Damage value for {row_text}:"
    Label(frame1, text = row_text).grid(row = row, column = 0)
    damages_entry[row].grid(row = row, column = 1)

  row = len(bodyparts)
  drop_text = "Damage drops (space separated):"
  Label(frame1, text = drop_text).grid(row = row, column = 0)
  drop_entry = Entry(frame1, width = 15)
  drop_entry.grid(row = row, column = 1)

  row += 1
  rate_text = "Weapon fire rate (shots per minute):"
  Label(frame1, text = rate_text).grid(row = row, column = 0)
  rate_entry = Entry(frame1, width = 15)
  rate_entry.grid(row = row, column = 1)

  result_text = Text(frame2, width = 0, height = 19)

  def show_results(results: str, width: int):
    result_text.pack(padx = 5, pady = 5)
    result_text.config(state = "normal", width = width)
    result_text.delete("1.0", "end")
    result_text.insert("end", results)
    result_text.config(state = "disabled")

  def parse_damage(value: str) -> float:
    value = value.strip()
    match = fullmatch(r"(\d+(?:\.\d+)?)\s*\*\s*(\d+(?:\.\d+)?)", value)
    if not match: return float(value)
    first_value, second_value = match.groups()
    return float(first_value) * float(second_value)

  def calculate():
    try:
      rate = float(rate_entry.get())
      damages = [parse_damage(e.get()) for e in damages_entry]
      drops = sorted((float(e) for e in drop_entry.get().split()), reverse = True)
      show_results(*get_table(damages, drops, rate))
    except Exception as e:
      show_results(f"Internal error occurred: {e!s}", 40)

  def focus_next(widget: Entry | Button):
    elem = widget.tk_focusNext()
    if elem: elem.focus_set()
    if isinstance(elem, Entry):
      elem.selection_range(0, "end")
    return "break"

  def copy_text(entry1: Entry, entry2: Entry):
    value = entry1.get()
    entry2.delete(0, "end")
    entry2.insert(0, value)
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
  calcbtn.grid(row = row, column = 0, columnspan = 2, pady = "0 10")
  calcbtn.bind("<Return>", lambda x: (focus_next(x.widget), calculate()))

  for child in frame1.winfo_children():
    child.grid_configure(padx = 5, pady = 5)

  root.mainloop()

#############################
###    RUNNING THE APP    ###
#############################

if __name__ == "__main__": main_interface(root = Tk())