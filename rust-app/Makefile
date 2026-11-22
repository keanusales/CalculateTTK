OPT_FLAGS = -Oz -flto=full -DNDEBUG -fdata-sections -ffunction-sections \
            -fvisibility=hidden -fvisibility-inlines-hidden -fno-plt \
            -fno-unwind-tables -fno-asynchronous-unwind-tables \
            -fno-semantic-interposition -fomit-frame-pointer \
            -fmerge-all-constants -fno-exceptions -fsplit-lto-unit \
            -fwhole-program-vtables -fvirtual-function-elimination

TARGET_BIN = ./target/x86_64-unknown-linux-gnu/release/delta_ttk

export CFLAGS = $(OPT_FLAGS)
export CXXFLAGS = $(OPT_FLAGS)
export CMAKE_C_FLAGS = $(OPT_FLAGS)
export CMAKE_CXX_FLAGS = $(OPT_FLAGS)
export CMAKE_C_FLAGS_MINSIZEREL = $(OPT_FLAGS)
export CMAKE_CXX_FLAGS_MINSIZEREL = $(OPT_FLAGS)
export CFLTK_WAYLAND_ONLY = 1

.PHONY: all release update move clean

all: release
deploy: update release move clean

release:
	@echo "Iniciando compilação release..."
	cargo clean && cargo build -Z build-std=std,panic_abort \
	-Z build-std-features=optimize_for_size --release \
	--target x86_64-unknown-linux-gnu --config .cargo/release.toml \
	&& strip --strip-section-headers --strip-all -R .comment \
	-R .gnu.build.attributes -R .annobin.notes -R .note.stapsdt \
	-R .note.gnu.build-id -R .note.ABI-tag $(TARGET_BIN)

update:
	@echo "Atualizando dependências..."
	cargo update && cargo clean

move:
	@echo "Movendo binário para ~/delta_ttk/delta_ttk..."
	mv $(TARGET_BIN) ~/delta_ttk/delta_ttk

clean:
	@echo "Limpando artefatos..."
	cargo clean