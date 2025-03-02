# Registry Analyzer

## Запуск через gui
- ```cargo run --release```

## Запуск через консоль с флагами **os, users, network, bios, hardware, dump**
- ```cargo run --release -- dump HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Lsa\Credssp```