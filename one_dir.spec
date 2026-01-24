# .spec para gerar o calculatettk em modo onedir

from pprint import pprint

global_name = "calculatettk"

analysis = Analysis(
  [f"{global_name}.py"],
  pathex = [],
  binaries = [],
  datas = [],
  hiddenimports = [],
  hookspath = [],
  hooksconfig = {},
  runtime_hooks = [],
  excludes = [
    # Unnecessary Python Imports
    "_bz2", "_collections_abc", "_ctypes", "_decimal", "_hashlib", "_lzma", "_socket", "_ssl", "_zstd",
    "_weakrefset", "abc", "argparse", "ast", "asyncio", "annotationlib", "base64", "bz2", "calendar",
    "codecs", "codeop", "compression", "compression._common", "compression._common._streams",
    "compression.zstd", "compression.zstd._zstdfile", "contextlib", "copy", "curses", "cv2",
    "dataclasses", "datetime", "difflib", "dis", "distutils", "email", "ftplib", "fnmatch",
    "genericpath", "getopt", "gettext", "gzip", "heapq", "html", "http", "idlelib", "imaplib",
    "inspect", "io", "linecache", "locale", "lzma", "matplotlib", "msilib", "msvcrt", "nntplib",
    "ntpath", "numpy", "os", "pandas", "pickle", "posixpath", "pprint", "pydoc", "pydoc_data", "PIL",
    "poplib", "quopri", "scipy", "select", "shutil", "smtpd", "smtplib", "spwd", "sre_compile",
    "sre_constants", "sre_parse", "stat", "stringprep", "struct", "subprocess", "tarfile", "telnetlib",
    "test", "textwrap", "threading", "token", "tokenize", "traceback", "tracemalloc", "typing",
    "unicodedata", "unittest", "urllib", "warnings", "weakref", "winreg", "winsound", "xml", "zipfile",

    # Unnecessary Encodings (left alone __init__, aliases, utf_8, _win_cp_codecs)
    "encodings.ascii", "encodings.base64_codec", "encodings.big5", "encodings.big5hkscs",
    "encodings.bz2_codec", "encodings.charmap", "encodings.cp037", "encodings.cp273", "encodings.cp424",
    "encodings.cp437", "encodings.cp500", "encodings.cp720", "encodings.cp737", "encodings.cp775",
    "encodings.cp850", "encodings.cp852", "encodings.cp855", "encodings.cp856", "encodings.cp857",
    "encodings.cp858", "encodings.cp860", "encodings.cp861", "encodings.cp862", "encodings.cp863",
    "encodings.cp864", "encodings.cp865", "encodings.cp866", "encodings.cp869", "encodings.cp874",
    "encodings.cp875", "encodings.cp932", "encodings.cp949", "encodings.cp950", "encodings.cp1006",
    "encodings.cp1026", "encodings.cp1125", "encodings.cp1140", "encodings.cp1250", "encodings.cp1251",
    "encodings.cp1252", "encodings.cp1253", "encodings.cp1254", "encodings.cp1255", "encodings.cp1256",
    "encodings.cp1257", "encodings.cp1258", "encodings.euc_jis_2004", "encodings.euc_jisx0213",
    "encodings.euc_jp", "encodings.euc_kr", "encodings.gb18030", "encodings.gb2312", "encodings.gbk",
    "encodings.hex_codec", "encodings.hp_roman8", "encodings.hz", "encodings.idna", "encodings.iso2022_jp",
    "encodings.iso2022_jp_1", "encodings.iso2022_jp_2", "encodings.iso2022_jp_2004",
    "encodings.iso2022_jp_3", "encodings.iso2022_jp_ext", "encodings.iso2022_kr",
    "encodings.iso8859_1", "encodings.iso8859_10", "encodings.iso8859_11", "encodings.iso8859_13",
    "encodings.iso8859_14", "encodings.iso8859_15", "encodings.iso8859_16", "encodings.iso8859_2",
    "encodings.iso8859_3", "encodings.iso8859_4", "encodings.iso8859_5", "encodings.iso8859_6",
    "encodings.iso8859_7", "encodings.iso8859_8", "encodings.iso8859_9", "encodings.johab",
    "encodings.koi8_r", "encodings.koi8_t", "encodings.koi8_u", "encodings.kz1048",
    "encodings.latin_1", "encodings.mac_arabic", "encodings.mac_croatian", "encodings.mac_cyrillic",
    "encodings.mac_farsi", "encodings.mac_greek", "encodings.mac_iceland", "encodings.mac_latin2",
    "encodings.mac_roman", "encodings.mac_romanian", "encodings.mac_turkish", "encodings.mbcs",
    "encodings.oem", "encodings.palmos", "encodings.ptcp154", "encodings.punycode",
    "encodings.quopri_codec", "encodings.raw_unicode_escape", "encodings.rot_13", "encodings.shift_jis",
    "encodings.shift_jis_2004", "encodings.shift_jisx0213", "encodings.tis_620", "encodings.undefined",
    "encodings.unicode_escape", "encodings.utf_16", "encodings.utf_16_be", "encodings.utf_16_le",
    "encodings.utf_32", "encodings.utf_32_be", "encodings.utf_32_le", "encodings.utf_7",
    "encodings.utf_8_sig", "encodings.uu_codec", "encodings.zlib_codec"
  ],
  noarchive = False,
  optimize = 2,
)

analysis.datas = [(src, dest, type) for (src, dest, type) in analysis.datas
  if not any(keyword in src.lower() for keyword in (
    "aquatheme.tcl", "bgerror.tcl", "choosedir", "clrpick", "clock.tcl", "comdlg",
    "console", "demos", "dialog", "doc", "encoding", "examples", "fontchooser",
    "history.tcl", "http1.0", "iconlist", "images", "megawidget.tcl", "mkpsenc.tcl",
    "msgbox.tcl", "msgs", "obsolete", "opt", "package.tcl", "palette.tcl", "parray.tcl",
    "pkgIndex.tcl", "safe.tcl", "safetk", "tarfile", "tcl8", "tcl8.5", "tcl8.6",
    "tearoff.tcl", "test", "term", "tix", "tix8.4", "tk8.6", "tkfbox", "tm.tcl",
    "tzdata", "unsupported", "word.tcl", "xmfbox"
  ))]

analysis.binaries = [(name, path, type) for (name, path, type) in analysis.binaries
  if any(include in name.lower() for include in ("python", "tcl", "tk", "zlib"))]

with open("analysis.txt", "w") as file: pprint(analysis.__dict__, file)

pyz = PYZ(analysis.pure)

exe = EXE(
  pyz,
  analysis.scripts,
  exclude_binaries = True,
  name = global_name,
  debug = False,
  strip = True,
  console = False,
  argv_emulation = False,
  target_arch = None,
  codesign_identity = None,
  entitlements_file = None,
)

coll = COLLECT(
  exe,
  analysis.binaries,
  analysis.datas,
  strip = True,
  name = global_name,
)