Name:           attack-shark-x3
Version:        1.0.0
Release:        1%{?dist}
Summary:        Native Linux configuration tool for Attack Shark X3 gaming mouse

License:        MIT
URL:            https://github.com/enthony/attacksharkx3-linux
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  hidapi-devel
BuildRequires:  libxkbcommon-devel
BuildRequires:  wayland-devel

Requires:       hidapi

%description
Native Linux graphical configuration utility for the Attack Shark X3 gaming mouse.
Supports DPI stages, polling rate, debounce time, sleep timers, button remapping,
macros, profiles, and live battery level.

%prep
%autosetup

%build
cargo build --release

%install
rm -rf %{buildroot}
install -D -p -m 0755 target/release/attack-shark-x3 %{buildroot}%{_bindir}/attack-shark-x3
install -D -p -m 0644 packaging/udev/99-attack-shark-x3.rules %{buildroot}%{_udevrulesdir}/99-attack-shark-x3.rules
install -D -p -m 0644 packaging/desktop/attack-shark-x3.desktop %{buildroot}%{_datadir}/applications/attack-shark-x3.desktop
install -D -p -m 0644 assets/icons/icon.png %{buildroot}%{_datadir}/icons/hicolor/256x256/apps/attack-shark-x3.png

%files
%license LICENSE
%doc README.md docs/
%{_bindir}/attack-shark-x3
%{_udevrulesdir}/99-attack-shark-x3.rules
%{_datadir}/applications/attack-shark-x3.desktop
%{_datadir}/icons/hicolor/256x256/apps/attack-shark-x3.png

%changelog
* Tue Oct 13 2026 Enthony <contact@example.com> - 1.0.0-1
- Initial native Linux release
