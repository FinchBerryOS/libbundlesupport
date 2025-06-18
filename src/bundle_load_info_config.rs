// Importiere Funktionen, Typen und Traits aus dem crate
use once_cell::sync::OnceCell;   // Für lazy, threadsichere Initialisierung (Singleton)
use std::path::Path;             // Für Dateipfade
use serde_json;
use crate::{get_current_launched_bundle_path, BundleError, BundleInfoConfigFile};                  // Zum Parsen von JSON

// Singleton-Instanz für BundleInfoConfigFile, die einmalig initialisiert wird
static MAIN_BUNDLE_INSTANCE: OnceCell<BundleInfoConfigFile> = OnceCell::new();

// Implementiere From-Conversion von std::io::Error zu BundleError::IoError
impl From<std::io::Error> for BundleError {
    fn from(e: std::io::Error) -> Self {
        BundleError::IoError(e)
    }
}

// Lädt und parst die Datei Content/Info.json im angegebenen Verzeichnis.
// Liefert bei Erfolg das BundleInfoConfigFile, sonst einen passenden BundleError.
fn load_bundle_info_file(path: &Path) -> Result<BundleInfoConfigFile, BundleError> {
    // Erzeuge vollständigen Pfad zu Content/Info.json
    let info_json_path = path.join("Content/Info.json");

    // Prüfe, ob die Datei existiert – sonst NotFound Fehler zurückgeben
    if !info_json_path.exists() {
        return Err(BundleError::NotFound(format!(
            "Info.json file not found at '{}'",
            info_json_path.display()
        )));
    }

    // Lese die Datei als Bytes ein
    let data = std::fs::read(&info_json_path).map_err(BundleError::IoError)?;

    // Versuche die gelesenen Daten als BundleInfoConfigFile zu parsen
    let bundle_info: BundleInfoConfigFile = serde_json::from_slice(&data).map_err(|e| {
        BundleError::InvalidFormat(format!("Failed to parse Info.json: {}", e))
    })?;

    // Bei Erfolg BundleInfoConfigFile zurückgeben
    Ok(bundle_info)
}

// Lädt das Bundle-Info-Config nur, wenn das Programm tatsächlich aus einem Bundle gestartet wurde.
// Gibt einen statischen Verweis auf die Singleton-Instanz zurück.
/// Lädt die Config ins Singleton, Fehler bei zweitem Aufruf
pub fn bundle_load_info_config() -> Result<&'static BundleInfoConfigFile, BundleError> {
    MAIN_BUNDLE_INSTANCE.get().map(|_cfg| {
        Err(BundleError::InvalidFormat("Config bereits geladen".into()))
    }).unwrap_or_else(|| {
        // Hier wie vorher:
        let current_bundle_path = match get_current_launched_bundle_path() {
            Ok(Some(bundle)) => bundle,
            Ok(None) => return Err(BundleError::InvalidFormat(
                "Bundle-Pfad konnte nicht ermittelt werden".to_string(),
            )),
            Err(e) => return Err(BundleError::InvalidFormat(
                format!("Fehler beim Bundle-Root: {:?}", e)
            )),
        };
        let bundle_config = load_bundle_info_file(&current_bundle_path)?;
        Ok(MAIN_BUNDLE_INSTANCE.get_or_init(|| bundle_config))
    })
}

/// Gibt nur das Config zurück, wenn bereits geladen, sonst Fehler
pub fn get_loaded_bundle_info_config() -> Result<&'static BundleInfoConfigFile, BundleError> {
    MAIN_BUNDLE_INSTANCE.get().ok_or(BundleError::NotLoaded)
}