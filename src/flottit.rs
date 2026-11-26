//! FlottIT Connect – fest einkompilierte Einstellungen.
//!
//! Diese Datei wird von `client/apply-branding.sh` (Repo flottit-connect) erzeugt.
//! Nicht von Hand ändern – Änderungen gehören in client/branding/templates/flottit.rs.
//!
//! Warum ein eigenes Modul statt `custom.txt`?
//! RustDesk liest Custom-Client-Einstellungen aus einer `custom.txt`, die mit dem
//! privaten Schlüssel von RustDesk signiert sein muss (nur über den kostenpflichtigen
//! Pro-Generator erhältlich). Wir setzen dieselben Tabellen (HARD_SETTINGS,
//! OVERWRITE_SETTINGS, BUILTIN_SETTINGS) stattdessen direkt beim Programmstart.
//! `apply()` läuft NACH dem Laden einer eventuellen `custom.txt` und gewinnt daher immer.
//!
//! Vorrang in RustDesk: OVERWRITE_* schlägt gespeicherte Benutzerwerte und Defaults
//! (siehe hbb_common::config::Config::get_option). Der Kunde kann diese Werte also
//! weder in der Oberfläche noch über die Konfigurationsdatei ändern.

use hbb_common::config;

/// ID-/Relay-Server (Hostname oder IP), von apply-branding.sh eingesetzt.
pub const FLOTTIT_HOST: &str = "connect.flottit.at";
/// Public Key des Servers (id_ed25519.pub), von apply-branding.sh eingesetzt.
pub const FLOTTIT_KEY: &str = "RkxPVFRJVC1URVNULUtFWS1OSUNIVC1WRVJXRU5ERU4=";
/// Anzeigename, Fenstertitel, Konfigurationsordner.
pub const FLOTTIT_APP_NAME: &str = "FlottIT Connect";
/// "kunde" = nur eingehend (FlottIT Connect), "techniker" = ausgehend (FlottIT Techniker)
pub const FLOTTIT_VARIANTE: &str = "kunde";

fn setze(map: &std::sync::RwLock<std::collections::HashMap<String, String>>, werte: &[(&str, &str)]) {
    let mut m = map.write().unwrap();
    for (k, v) in werte {
        m.insert((*k).to_owned(), (*v).to_owned());
    }
}

/// Server und Key fest – gilt für beide Varianten.
const SERVER: &[(&str, &str)] = &[
    ("custom-rendezvous-server", FLOTTIT_HOST),
    ("relay-server", FLOTTIT_HOST),
    ("key", FLOTTIT_KEY),
    // Keine Verbindung zu Fremdservern über WebSocket/WebRTC-Umwege
    ("allow-websocket", "N"),
    ("allow-auto-update", "N"),
];

/// Kundenvariante: harte Sperren (HARD_SETTINGS werden von RustDesk z. B. über
/// config::is_incoming_only() / is_disable_installation() ausgewertet).
const KUNDE_HART: &[(&str, &str)] = &[
    ("conn-type", "incoming"),
    ("disable-installation", "Y"),
    ("disable-settings", "Y"),
    ("disable-ab", "Y"),
    ("disable-account", "Y"),
    ("disable-tcp-listen", "Y"),
];

/// Kundenvariante: Server-Optionen, die der Kunde nicht ändern kann.
const KUNDE_OPTIONEN: &[(&str, &str)] = &[
    // Zustimmung nur per Klick, kein Passwort-Zugang
    ("approve-mode", "click"),
    ("verification-method", "use-temporary-password"),
    // Standardmäßig AUS (pro Sitzung im Zulassen-Fenster freischaltbar)
    ("enable-file-transfer", "N"),
    ("enable-audio", "N"),
    ("enable-camera", "N"),
    ("enable-terminal", "N"),
    ("enable-record-session", "N"),
    ("enable-tunnel", "N"),
    ("enable-remote-restart", "N"),
    ("enable-remote-printer", "N"),
    ("enable-privacy-mode", "N"),
    ("enable-block-input", "N"),
    ("allow-auto-record-incoming", "N"),
    // Fernänderung der Konfiguration verboten
    ("allow-remote-config-modification", "N"),
    // Kein Zugriff, wenn das Fenster geschlossen ist
    ("allow-only-conn-window-open", "Y"),
    // Keine Direktverbindungen/LAN-Erkennung, nur über unseren Server
    ("direct-server", "N"),
    ("enable-lan-discovery", "N"),
    ("enable-trusted-devices", "N"),
];

/// Kundenvariante: eingebaute Optionen (Oberfläche).
const KUNDE_BUILTIN: &[(&str, &str)] = &[
    ("enable-perm-change-in-accept-window", "Y"),
    ("hide-powered-by-me", "Y"),
    ("hide-security-settings", "Y"),
    ("hide-network-settings", "Y"),
    ("hide-server-settings", "Y"),
    ("hide-proxy-settings", "Y"),
    ("hide-websocket-settings", "Y"),
    ("hide-remote-printer-settings", "Y"),
    ("disable-change-permanent-password", "Y"),
    ("disable-change-id", "Y"),
    ("allow-deep-link-password", "N"),
    ("allow-deep-link-server-settings", "N"),
];

/// Lokale Optionen: helle Oberfläche, Deutsch, keine Update-Prüfung bei rustdesk.com.
const LOKAL: &[(&str, &str)] = &[
    ("theme", "light"),
    ("lang", "de"),
    ("enable-check-update", "N"),
];

/// Technikervariante: nur ausgehend, eingehende Verbindungen nicht möglich.
const TECHNIKER_HART: &[(&str, &str)] = &[
    ("conn-type", "outgoing"),
    ("disable-installation", "N"),
    ("disable-tcp-listen", "Y"),
];

/// Wird beim Start jedes RustDesk-Prozesses aufgerufen (über load_custom_client).
pub fn apply() {
    *config::APP_NAME.write().unwrap() = FLOTTIT_APP_NAME.to_owned();
    // Dateiname-Konfiguration (rustdesk-host=...) ist im Fork deaktiviert, zur Sicherheit leeren.
    config::EXE_RENDEZVOUS_SERVER.write().unwrap().clear();
    *config::PROD_RENDEZVOUS_SERVER.write().unwrap() = FLOTTIT_HOST.to_owned();
    setze(&config::OVERWRITE_SETTINGS, SERVER);
    setze(&config::OVERWRITE_LOCAL_SETTINGS, LOKAL);
    if FLOTTIT_VARIANTE == "techniker" {
        setze(&config::HARD_SETTINGS, TECHNIKER_HART);
    } else {
        setze(&config::HARD_SETTINGS, KUNDE_HART);
        setze(&config::OVERWRITE_SETTINGS, KUNDE_OPTIONEN);
        setze(&config::BUILTIN_SETTINGS, KUNDE_BUILTIN);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_und_key_sind_gesetzt() {
        assert!(!FLOTTIT_HOST.contains("@@"));
        assert!(!FLOTTIT_HOST.contains("rustdesk.com"));
        assert_eq!(FLOTTIT_KEY.len(), 44);
    }

    #[test]
    fn apply_setzt_sperren() {
        apply();
        assert_eq!(config::Config::get_option("custom-rendezvous-server"), FLOTTIT_HOST);
        assert_eq!(config::Config::get_option("key"), FLOTTIT_KEY);
        assert_eq!(config::Config::get_rendezvous_server(), format!("{}:21116", FLOTTIT_HOST));
        // Benutzer versucht, einen fremden Server zu setzen -> bleibt unser Server
        config::Config::set_option("custom-rendezvous-server".into(), "fremder-server.example".into());
        assert_eq!(config::Config::get_option("custom-rendezvous-server"), FLOTTIT_HOST);
        if FLOTTIT_VARIANTE != "techniker" {
            assert!(config::is_incoming_only());
            assert!(config::is_disable_installation());
            assert_eq!(config::Config::get_option("approve-mode"), "click");
            assert_eq!(config::Config::get_option("enable-file-transfer"), "N");
            config::Config::set_option("enable-file-transfer".into(), "Y".into());
            assert_eq!(config::Config::get_option("enable-file-transfer"), "N");
        } else {
            assert!(config::is_outgoing_only());
        }
    }
}
