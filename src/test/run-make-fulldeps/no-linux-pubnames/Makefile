-include ../tools.mk

# Test that -g doesn't emit .debug_pubnames on Linux.

all:
ifeq ($(UNAME),Linux)
	$(RUSTC) -g main.rs
	! readelf -WS $(TMPDIR)/main | grep -q debug_pubnames
endif
