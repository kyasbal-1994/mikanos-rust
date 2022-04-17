MOUNT_POINT=./mnt
BOOTLOADER_LOCATION=./bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi
KERNEL_LOCATION=./kernel/target/x86_64-unknown-none-mikanos-rust/debug/kernel.elf
DISK_IMAGE_LOCATION=./build.img
DISK_IMAGE_SIZE=200M
MEMORY_SIZE=1G

.PHONY: run
run: $(DISK_IMAGE_LOCATION) ovmf/OVMF_CODE.fd ovmf/OVMF_VARS.fd
	set +e
	qemu-system-x86_64 -smp 4 -m $(MEMORY_SIZE) \
		-drive if=pflash,format=raw,readonly=on,file=./ovmf/OVMF_CODE.fd \
		-drive if=pflash,format=raw,file=./ovmf/OVMF_VARS.fd \
		-drive if=none,id=drive0,format=raw,file=$(DISK_IMAGE_LOCATION) \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-device virtio-blk-pci,drive=drive0 \
		-serial stdio

$(DISK_IMAGE_LOCATION): $(BOOTLOADER_LOCATION) $(KERNEL_LOCATION)
	qemu-img create -f raw $(DISK_IMAGE_LOCATION) $(DISK_IMAGE_SIZE)
	mkfs.fat -n 'MikanOSRust' -s 2 -f 2 -R 32 -F 32 $(DISK_IMAGE_LOCATION)
	mkdir -p $(MOUNT_POINT)
	sudo mount -o loop $(DISK_IMAGE_LOCATION) $(MOUNT_POINT)
	sudo mkdir -p $(MOUNT_POINT)/EFI/BOOT
	sudo cp $(BOOTLOADER_LOCATION) $(MOUNT_POINT)/EFI/BOOT/BOOTX64.EFI
	sudo cp $(KERNEL_LOCATION) $(MOUNT_POINT)/kernel.elf
	sleep 0.5
	sudo umount $(MOUNT_POINT)


.PHONY: build
build: $(BOOTLOADER_LOCATION) $(KERNEL_LOCATION)

$(BOOTLOADER_LOCATION): ./bootloader/src/*.rs ./bootloader/Cargo.toml ./bootloader/.cargo/*
	cd ./bootloader && cargo build

$(KERNEL_LOCATION): ./kernel/src/*.rs ./kernel/Cargo.toml ./kernel/.cargo/* ./kernel/x86_64-unknown-none-mikanos-rust.json
	cd ./kernel && cargo build

ovmf/OVMF_CODE.fd:
	cp /usr/share/OVMF/OVMF_CODE.fd ovmf/OVMF_CODE.fd

ovmf/OVMF_VARS.fd:
	cp  /usr/share/OVMF/OVMF_VARS.fd ovmf/OVMF_VARS.fd