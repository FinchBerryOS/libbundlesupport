use serde::{Deserialize, Deserializer};
use std::fmt;


#[derive(Debug, Clone)]
pub enum EntitlementType {
    // -------- Multimedia & Files ----------

    /// Zugriff auf die Kamera (Foto/Video-Aufnahmen, QR-Scan)
    Camera,
    /// Zugriff auf das Mikrofon (Audio-Aufnahmen, Sprachbefehle)
    Microphone,
    /// Zugriff auf Standort/GPS-Daten (Maps, Geofencing, Location-based Services)
    Location,
    /// Zugriff auf Fotos/Bildergalerie (Bilder anzeigen/hochladen/speichern)
    Photos,
    /// Zugriff auf das Dateisystem (Dateien öffnen/speichern/bearbeiten)
    Files,
    /// Zugriff auf die Zwischenablage (Copy/Paste von Daten)
    Clipboard,
    /// Aufnahme oder Teilen des Bildschirms (Screen Recording/Sharing)
    ScreenRecording,

    // -------- Sensors ----------
    /// Zugriff auf Beschleunigungssensor (Schrittzähler, Bewegungserkennung)
    Accelerometer,
    /// Zugriff auf Gyroskop (Orientierung, Bewegungssteuerung)
    Gyroscope,
    /// Zugriff auf Magnetometer (Kompass, Navigation)
    Magnetometer,
    /// Zugriff auf Barometer (Höhenerkennung, Wetterdaten)
    Barometer,
    /// Zugriff auf Näherungssensor (Annäherungserkennung, Displayabschaltung)
    ProximitySensor,
    /// Zugriff auf Umgebungslichtsensor (Display-Helligkeit anpassen)
    AmbientLightSensor,
    /// Zugriff auf Temperatursensor (Gerätetemperatur, Umgebung)
    TemperatureSensor,
    /// Zugriff auf Luftfeuchtigkeitssensor (Klimadaten, Smart Home)
    HumiditySensor,
    /// Zugriff auf Herzfrequenzsensor (Fitness, Health-Tracking)
    HeartRateSensor,

    // -------- Network & Devices ----------
    /// Firewall- und Netzwerkregelverwaltung (Ports, Zugriffssteuerung)
    Firewall,
    /// Allgemeiner Netzwerkzugriff (Internet, lokale Netzwerke)
    Network,
    /// Zugriff auf Bluetooth (Gerätesuche, -kopplung, Datenübertragung)
    Bluetooth,
    /// Zugriff auf USB-Geräte (Datenträger, Hardware Wallets, Peripherie)
    Usb,
    /// Zugriff auf NFC-Funktionalitäten (Kontaktloses Bezahlen, Authentifizierung)
    Nfc,

    // -------- Virtualization & Containers ----------
    /// Erstellen oder Steuern von virtuellen Maschinen (VMs)
    VirtualMachine,
    /// Nutzung und Verwaltung von Containern (z.B. Docker, App-Sandboxing)
    Container,
    /// Nutzung von virtuellen Dateisystemen (gemountete Dateisysteme, FUSE)
    VirtualFileSystem,

    // -------- System Notifications ----------
    /// Versand und Empfang von System-Benachrichtigungen
    Notification,

    // -------- Script & Engine Support ----------
    /// Ausführen von Python-Skripten (Automatisierung, Plugins)
    PythonIO,
    /// Ausführen von Bash-Skripten (Automatisierung, Systemintegration)
    BashIO,
    /// Ausführen von Windows-Anwendungen mittels Wine-Engine
    WineEngine,

    // -------- System Permissions ----------
    /// Zugriff auf geschützte Systemdienste (z.B. Prozesssteuerung, Power-Management)
    SystemServices,

    // -------- VPN Permissions ----------
    /// Betrieb als VPN-Host (VPN-Serverfunktion)
    VpnHost,
    /// Verbindung als VPN-Client (VPN-Clientfunktion)
    VpnClient,

    // -------- Crypto/Bitcoin Permissions ----------
    /// Verwalteter Zugang zu kryptographischem Speicher (Keys, Seeds, Zertifikate)
    CryptoStore,
    /// Lesender Zugriff auf Wallet-Daten (Adressen, Guthaben, Transaktionsverlauf)
    WalletRead,
    /// Schreibender Zugriff auf Wallet (Transaktionen erstellen/senden)
    WalletWrite,
    /// Zugriff auf Blockchain-Daten (Blöcke, Transaktionen)
    ChainRead,
    /// Blockchain-Synchronisierung, vollständige Verifizierung
    ChainSync,
    /// Zugriff auf P2P-Netzwerkfunktionen (Node Discovery, Gossip, Peer-Management)
    P2PNetwork,
    /// Zugriff auf den Mempool (ungeminte Transaktionen lesen/schreiben)
    MempoolAccess,
    /// Verwaltung von Lightning-Channels (öffnen, schließen, Status abfragen)
    LightningChannels,
    /// Lightning-Zahlungen senden/empfangen
    LightningPay,
    /// Lesender Zugriff auf Lightning-Netzwerk-Info (Channels, Routing)
    LightningInfo,
    /// Export des Wallet-Seeds/Keys (Backup, Migration – **sehr sensitiv**)
    KeySeedExport,
    /// Zugriff auf externe Hardware Wallets (z.B. Ledger, Trezor)
    HardwareWalletAccess,
    /// Lesender Zugriff auf Hardware Wallet (z.B. Adressen, Public Keys)
    HardwareWalletRead,
    /// Signatur-Funktionen auf Hardware Wallet (Transaktionen signieren)
    HardwareWalletSign,

    // -------- Enterprise Permissions ----------
    // Device & App Management
    /// Administrative Steuerung des Geräts (MDM, Richtlinien setzen)
    DeviceAdmin,
    /// Fernlöschung von Daten oder Gerät (bei Verlust, Diebstahl)
    RemoteWipe,
    /// Fernkonfiguration von Einstellungen/Profilen (z.B. WLAN, VPN, App-Settings)
    RemoteConfig,
    /// Installation/Deinstallation von Apps durch die Verwaltung
    AppInstall,

    // Network & Connectivity
    /// Zugang zu firmeninternen VPNs und deren Verwaltung
    EnterpriseVpn,
    /// Zentrale Steuerung und Einschränkung von Netzwerkzugriffen
    NetworkPolicyControl,

    // Auth & Identity
    /// Nutzung von Single Sign-On (LDAP, SAML, OIDC, Azure AD, etc.)
    EnterpriseSSO,
    /// Zugriff auf oder Import von Zertifikaten (TLS, E-Mail, VPN)
    CertificateStore,
    /// Verwaltung und Erzwingung von biometrischer Authentifizierung (Face/TouchID)
    BiometricAdmin,

    // Security & DLP
    /// Steuerung von Datenabfluss (z.B. Screenshots, USB, Weiterleitungen blockieren)
    DlpControl,
    /// Kontrolle/Absicherung der Zwischenablage (Clipboard) im Enterprise-Kontext
    SecureClipboard,
    /// Zugriff auf zentrale Audit- und Sicherheitsprotokolle
    AuditLogAccess,
    /// Auswertung der App- und Gerätenutzung (anonymisiert/aggregiert)
    UsageStats,

    // Updates & Support
    /// Erlaubnis, App/Client eigenständig zu updaten (Self-Update)
    SelfUpdate,
    /// Remote-Support und Fernzugriff (z.B. für IT-Support, temporär und autorisiert)
    RemoteSupport,

    // -------- Raspberry Pi / Embedded Hardware Permissions ----------
    /// Zugriff auf General Purpose Input/Output Pins (z.B. LED schalten, Sensoren auslesen)
    GpioAccess,
    /// Zugriff auf I2C-Bus (Kommunikation mit z.B. Displays, Sensoren, RTC)
    I2cAccess,
    /// Zugriff auf SPI-Bus (z.B. für schnelle Sensoren, Flash-Speicher)
    SpiAccess,
    /// Zugriff auf UART/Serielle Schnittstellen (z.B. serielle Kommunikation mit Modulen)
    UartAccess,
    /// Zugriff auf PWM-Ausgänge (z.B. für Motorsteuerung, LED-Dimmung)
    PwmAccess,
    /// Zugriff auf 1-Wire-Bus (z.B. für Temperatursensoren)
    OneWireAccess,
    /// Zugriff auf CAN-Bus (z.B. für Automobil- oder Industrieanwendungen)
    CanBusAccess,
    /// Zugriff auf ADC (Analog-Digital-Converter, falls vorhanden)
    AdcAccess,
    /// Zugriff auf integriertes Display (z.B. Pi Touchscreen)
    DisplayAccess,
    /// Zugriff auf Kamera-Modul über CSI (Camera Serial Interface)
    PiCameraModule,
    /// Zugriff auf Hardware-Temperatursensor des Boards
    BoardTemperatureAccess,
    /// Zugriff auf Board-spezifische LEDs (z.B. „Power“, „ACT“ LED)
    BoardLedAccess,
}

impl EntitlementType {
    pub fn as_str(&self) -> &'static str {
        match self {
            // Multimedia & Files
            EntitlementType::Camera => "camera",
            EntitlementType::Microphone => "microphone",
            EntitlementType::Location => "location",
            EntitlementType::Photos => "photos",
            EntitlementType::Files => "files",
            EntitlementType::Clipboard => "clipboard",
            EntitlementType::ScreenRecording => "screenrecording",

            // Sensors
            EntitlementType::Accelerometer => "accelerometer",
            EntitlementType::Gyroscope => "gyroscope",
            EntitlementType::Magnetometer => "magnetometer",
            EntitlementType::Barometer => "barometer",
            EntitlementType::ProximitySensor => "proximitysensor",
            EntitlementType::AmbientLightSensor => "ambientlightsensor",
            EntitlementType::TemperatureSensor => "temperaturesensor",
            EntitlementType::HumiditySensor => "humiditysensor",
            EntitlementType::HeartRateSensor => "heartratesensor",

            // Network & Devices
            EntitlementType::Firewall => "firewall",
            EntitlementType::Network => "network",
            EntitlementType::Bluetooth => "bluetooth",
            EntitlementType::Usb => "usb",
            EntitlementType::Nfc => "nfc",

            // Virtualization & Containers
            EntitlementType::VirtualMachine => "virtualmachine",
            EntitlementType::Container => "container",
            EntitlementType::VirtualFileSystem => "virtualfilesystem",

            // System Notifications
            EntitlementType::Notification => "notification",

            // Script & Engine Support
            EntitlementType::PythonIO => "pythonio",
            EntitlementType::BashIO => "bashio",
            EntitlementType::WineEngine => "wineengine",

            // System Permissions
            EntitlementType::SystemServices => "systemservices",

            // VPN
            EntitlementType::VpnHost => "vpnhost",
            EntitlementType::VpnClient => "vpnclient",

            // Crypto/Bitcoin
            EntitlementType::CryptoStore => "cryptostore",
            EntitlementType::WalletRead => "walletread",
            EntitlementType::WalletWrite => "walletwrite",
            EntitlementType::ChainRead => "chainread",
            EntitlementType::ChainSync => "chainsync",
            EntitlementType::P2PNetwork => "p2pnetwork",
            EntitlementType::MempoolAccess => "mempoolaccess",
            EntitlementType::LightningChannels => "lightningchannels",
            EntitlementType::LightningPay => "lightningpay",
            EntitlementType::LightningInfo => "lightninginfo",
            EntitlementType::KeySeedExport => "keyseedexport",
            EntitlementType::HardwareWalletAccess => "hardwarewalletaccess",
            EntitlementType::HardwareWalletRead => "hardwarewalletread",
            EntitlementType::HardwareWalletSign => "hardwarewalletsign",

            // Enterprise: Device & App Management
            EntitlementType::DeviceAdmin => "deviceadmin",
            EntitlementType::RemoteWipe => "remotewipe",
            EntitlementType::RemoteConfig => "remoteconfig",
            EntitlementType::AppInstall => "appinstall",

            // Enterprise: Network & Connectivity
            EntitlementType::EnterpriseVpn => "enterprisevpn",
            EntitlementType::NetworkPolicyControl => "networkpolicycontrol",

            // Enterprise: Auth & Identity
            EntitlementType::EnterpriseSSO => "enterprisesso",
            EntitlementType::CertificateStore => "certificatestore",
            EntitlementType::BiometricAdmin => "biometricadmin",

            // Enterprise: Security & DLP
            EntitlementType::DlpControl => "dlpcontrol",
            EntitlementType::SecureClipboard => "secureclipboard",
            EntitlementType::AuditLogAccess => "auditlogaccess",
            EntitlementType::UsageStats => "usagestats",

            // Enterprise: Updates & Support
            EntitlementType::SelfUpdate => "selfupdate",
            EntitlementType::RemoteSupport => "remotesupport",

            // Raspberry Pi / Embedded Hardware
            EntitlementType::GpioAccess => "gpioaccess",
            EntitlementType::I2cAccess => "i2caccess",
            EntitlementType::SpiAccess => "spiaccess",
            EntitlementType::UartAccess => "uartaccess",
            EntitlementType::PwmAccess => "pwmaccess",
            EntitlementType::OneWireAccess => "onewireaccess",
            EntitlementType::CanBusAccess => "canbusaccess",
            EntitlementType::AdcAccess => "adcaccess",
            EntitlementType::DisplayAccess => "displayaccess",
            EntitlementType::PiCameraModule => "picameramodule",
            EntitlementType::BoardTemperatureAccess => "boardtemperatureaccess",
            EntitlementType::BoardLedAccess => "boardledaccess",
        }
    }
}

// Custom deserializer, um Strings in EntitlementType umzuwandeln
impl<'de> Deserialize<'de> for EntitlementType {
    fn deserialize<D>(deserializer: D) -> Result<EntitlementType, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntitlementTypeVisitor;

        impl<'de> serde::de::Visitor<'de> for EntitlementTypeVisitor {
            type Value = EntitlementType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid permission type string")
            }

            fn visit_str<E>(self, value: &str) -> Result<EntitlementType, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "camera" => Ok(EntitlementType::Camera),
                    "microphone" => Ok(EntitlementType::Microphone),
                    "location" => Ok(EntitlementType::Location),
                    // ... weitere Mappings ...
                    _ => Err(E::custom(format!("unknown permission type: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(EntitlementTypeVisitor)
    }
}