del /Q /F ".\build\*"

cargo build --package=bootloader --target=[bits16].json
cargo build --package=stage2 --target=[bits16].json
cargo build --package=stage3 --target=[bits32].json
cargo build --package=kernel --target=[bits32].json
cargo build --package=userland --target=[bits32].json

wsl sh -c "objcopy -I elf32-i386 -O binary target/[bits16]/debug/bootloader build/bootloader.bin"
wsl sh -c "objcopy -I elf32-i386 -O binary target/[bits16]/debug/stage2 build/stage2.bin"
wsl sh -c "objcopy -I elf32-i386 -O binary target/[bits32]/debug/stage3 build/stage3.bin"
wsl sh -c "objcopy -I elf32-i386 -O binary target/[bits32]/debug/kernel build/kernel.bin"
wsl sh -c "objcopy -I elf32-i386 -O binary target/[bits32]/debug/userland build/userland.bin"

wsl dd if=/dev/zero of=build/disk.img bs=512 count=131072

wsl dd if=build/bootloader.bin of=build/disk.img conv=notrunc
wsl dd if=build/stage2.bin of=build/disk.img bs=512 seek=2048 conv=notrunc
wsl dd if=build/stage3.bin of=build/disk.img bs=512 seek=3072 conv=notrunc
wsl dd if=build/kernel.bin of=build/disk.img bs=512 seek=4096 conv=notrunc
wsl dd if=build/userland.bin of=build/disk.img bs=512 seek=8192 conv=notrunc

qemu-system-i386 -hda ".\build\disk.img" -m 1G -serial stdio