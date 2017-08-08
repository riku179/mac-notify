NIC=${NIC_NAME}

.PHONY: all build
all:
	cargo build
	sudo setcap cap_net_raw,cap_net_admin=eip ./target/debug/mac-notify
	./target/debug/mac-notify $(NIC)

build:
	sudo setcap cap_net_raw,cap_net_admin=eip ./target/debug/mac-notify
	./target/debug/mac-notify $(NIC)
