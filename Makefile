.PHONY: all build
all:
	cargo build
	sudo setcap cap_net_raw,cap_net_admin=eip ./target/debug/mac_parse
	./target/debug/mac_parse enp2s0f0

buld:
	sudo setcap cap_net_raw,cap_net_admin=eip ./target/debug/mac_parse
	./target/debug/mac_parse enp2s0f0
