# FlottIT Connect (FlottIT-Fork von RustDesk)

Dieser Zweig ist eine angepasste Version von [RustDesk](https://github.com/rustdesk/rustdesk)
**1.5.3** und steht wie das Original unter der **GNU AGPL-3.0** (siehe `LICENCE`).
Alle Copyright-Hinweise von RustDesk bleiben unverändert erhalten.

Erzeugt automatisch mit `client/apply-branding.sh` aus dem FlottIT-Projekt (Build 2a9bfba-TEST).

Änderungen gegenüber RustDesk 1.5.3:

- Fest eingestellter eigener ID-/Relay-Server `connect.flottit.at` und dessen Public Key
  (`libs/hbb_common/src/config.rs`, `src/flottit.rs`). Öffentliche RustDesk-Server entfernt.
- Variante `kunde`: Sicherheits-Voreinstellungen in `src/flottit.rs`
  (nur eingehend, Zustimmung per Klick, Rechte standardmäßig aus, keine Installation).
- Server-Konfiguration über den Dateinamen deaktiviert (`src/platform/windows.rs`).
- Name, Icons, Farben, vereinfachtes Startfenster (`flutter/lib/flottit/`), deutsche Texte.
- Nur der Windows-Build-Workflow `.github/workflows/flottit-windows.yml`.

Quellcode dieser Version: https://github.com/dgn57/flottit-rustdesk/tree/flottit
