# .spec para gerar o calculatettk em modo onefile

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
        "_ssl", "_hashlib", "_decimal", "_bz2", "_lzma", "_zstd",
        "email", "http", "xml", "html", "urllib", "unittest",
        "test", "distutils", "pydoc_data", "select", "unicodedata"
    ],
    noarchive = False,
    optimize = 2,
)

denied_folders = ("tzdata", "encoding", "msgs", "demos", "images", "opt", "tcl8", "http1.0")

analysis.datas = [(src, dest, type) for (src, dest, type) in analysis.datas
    if not (any(keyword in src for keyword in denied_folders) or src.endswith(".terms"))]

pyz = PYZ(analysis.pure)

exe = EXE(
    pyz,
    analysis.scripts,
    analysis.binaries,
    analysis.datas,
    [("O", None, "OPTION"), ("O", None, "OPTION")],
    name = global_name,
    debug = False,
    bootloader_ignore_signals = False,
    strip = True,
    upx = True,
    upx_exclude = ["libcrypto-3.dll", "libssl-3.dll"],
    runtime_tmpdir = None,
    console = False,
    disable_windowed_traceback = False,
    argv_emulation = False,
    target_arch = None,
    codesign_identity = None,
    entitlements_file = None,
)