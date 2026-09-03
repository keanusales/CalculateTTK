OPT_FLAGS = -Oz -flto=full -fdata-sections -ffunction-sections -fvisibility=hidden -fno-plt \
						-fvisibility-inlines-hidden -fno-semantic-interposition -fomit-frame-pointer \
						-fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables

export CFLAGS = $(OPT_FLAGS)
export CXXFLAGS = $(OPT_FLAGS)

.PHONY: all release update clean

all: release

release:
	@echo "Iniciando compilação release..."
	cargo clean && cargo build --release --target x86_64-unknown-linux-gnu && strip ./target/x86_64-unknown-linux-gnu/release/delta_ttk

update:
	@echo "Atualizando dependências..."
	cargo update && cargo clean

clean:
	@echo "Limpando artefatos..."
	cargo clean