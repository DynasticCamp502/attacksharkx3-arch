# Attack Shark X3 — Native Linux Driver & GUI

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20(Wayland%20%2F%20X11)-blue.svg)]()

Uma implementação nativa, independente e open source em **Rust** para configuração completa do mouse **Attack Shark X3** no Linux, sem depender de Wine ou de softwares proprietários Windows.

---

## 🚀 Funcionalidades

- **Configuração de DPI**:
  - Ajuste de estágios de DPI (50 a 26.000 DPI em passos de 50).
  - Ativação/desativação e seleção de estágio ativo.
  - Indicadores de cor RGB individuais para cada nível de DPI.
- **Desempenho e Sensor (PixArt PAW3395)**:
  - Taxa de polling: 125 Hz, 250 Hz, 500 Hz e 1000 Hz.
  - Lift-Off Distance (LOD): 1 mm e 2 mm.
  - Tempo de resposta dos botões (Debounce): 2 a 50 ms.
  - Ripple Control (On/Off).
  - Angle Snapping (On/Off).
  - Motion Sync (On/Off).
- **Gerenciamento de Energia**:
  - Tempo de suspensão (Sleep): 0.5 a 30 minutos.
  - Tempo de suspensão profunda (Deep Sleep): 1 a 60 minutos.
  - Leitura do nível de bateria e voltagem em tempo real via HID.
- **Remapeamento de Botões**:
  - Configuração individual dos 5 botões físicos (Clique Esquerdo, Clique Direito, Clique Central, Avançar e Voltar).
  - Funções multimídia, navegação web, ciclar DPI e atalhos customizados (combinações de teclas).
- **Macros de Hardware**:
  - Criação de sequências de teclas e gravação no firmware do mouse (Slot 2).
  - Execução direta pelo hardware sem depender de injeção de software no sistema.
- **Perfis e Persistência**:
  - Criação, alternância, exclusão e restauração de perfis.
  - Persistência automática em `~/.config/attack-shark-x3/config.toml`.
- **Interface Gráfica Nativa**:
  - Desenvolvida em `egui/eframe`, compatível com Wayland e X11 (GNOME, KDE Plasma, etc.).
  - Reprodução visual fiel à interface original DuiLib.

---

## 🛠️ Instalação e Execução

### 1. Pré-requisitos (Fedora / Ubuntu / Debian / Arch)

- **Fedora**:
  ```bash
  sudo dnf install -y rust cargo hidapi-devel
  ```
- **Ubuntu / Debian**:
  ```bash
  sudo apt install -y rustc cargo libhidapi-dev
  ```
- **Arch Linux**:
  ```bash
  sudo pacman -S rust hidapi
  ```

### 2. Configurar Permissões (Regras Udev)

Para permitir que o aplicativo se comunique com o mouse sem precisar de `sudo`:

```bash
sudo cp packaging/udev/99-attack-shark-x3.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

### 3. Compilar e Executar

```bash
# Compilar versão otimizada
cargo build --release

# Executar a interface gráfica
./target/release/attack-shark-x3
```

---

## 📦 Estrutura do Projeto

```text
attacksharkx3-linux/
├── Cargo.toml
├── README.md
├── LICENSE
├── assets/                  # Ícones e recursos visuais originais
├── docs/
│   ├── reverse-engineering.md  # Relatório detalhado da engenharia reversa
│   ├── usb-protocol.md         # Especificação completa do protocolo HID
│   └── compatibility.md        # Compatibilidade com distros e ambientes
├── packaging/
│   ├── udev/                   # Regras udev (99-attack-shark-x3.rules)
│   ├── desktop/                # Atalho .desktop para menu do sistema
│   ├── deb/                    # Script para gerar pacote .deb
│   ├── rpm/                    # Arquivo spec para gerar pacote .rpm
│   └── appimage/               # Script para gerar AppImage
├── scripts/
│   └── install-udev.sh         # Script auxiliar de instalação de permissões
├── src/
│   ├── main.rs                 # Ponto de entrada do executável
│   ├── lib.rs                  # Biblioteca Rust reusável
│   ├── app.rs                  # Loop principal da aplicação eframe
│   ├── protocol/               # Codificação e decodificação dos pacotes HID
│   │   ├── constants.rs
│   │   ├── checksum.rs         # Cálculo do checksum de 16 bits big-endian
│   │   ├── sensor.rs           # Relatório 0x04 (DPI, LOD, sensor)
│   │   ├── power.rs            # Relatório 0x05 (Timers de sono e debounce)
│   │   ├── polling.rs          # Relatório 0x06 (Taxa de polling)
│   │   ├── buttons.rs          # Relatório 0x08 (Mapeamento de 18 slots)
│   │   ├── macro_upload.rs     # Relatório 0x09 (Upload de blocos de macro)
│   │   └── battery.rs          # Relatório 0x03 (Leitura de bateria/voltagem)
│   ├── device/                 # Detecção e comunicação HID via hidapi
│   ├── dpi/                    # Regras de negócio e validação de DPI
│   ├── buttons/                # Ações e remapeamento de botões
│   ├── macros/                 # Gerenciador de macros
│   ├── profiles/               # Gerenciador de perfis
│   ├── config/                 # Persistência em TOML
│   └── gui/                    # Componentes visuais egui/eframe
└── tests/
    └── protocol_tests.rs       # Testes automatizados de protocolo e checksums
```

---

## 🧪 Testes Automatizados

O projeto inclui testes de unidade e integração que validam cada byte dos pacotes construídos contra capturas reais do software de referência:

```bash
cargo test
```

---

## 📄 Licença

Distribuído sob a licença [MIT](LICENSE).
