NIC=${NIC_NAME}

.PHONY: all build
all:
	@cargo build
	@sudo setcap cap_net_raw,cap_net_admin=eip ./target/debug/mac-notify
	@./target/debug/mac-notify $(NIC)

run:
	@sudo setcap cap_net_raw,cap_net_admin=eip ./target/debug/mac-notify
	@./target/debug/mac-notify $(NIC)

check:
	cargo check
