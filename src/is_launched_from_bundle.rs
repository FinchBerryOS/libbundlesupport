use crate::{find_bundle_root, BundleError};

/// Prüft, ob das aktuell laufende Programm aus einem Bundle heraus gestartet wurde.
/// Gibt bei Erfolg `Ok(true)` zurück, falls ein Bundle-Root gefunden wurde, sonst `Ok(false)`.
/// Gibt bei einem Fehler (z.B. Zugriff auf Programmpfad nicht möglich) ein entsprechendes BundleError zurück.
pub fn is_launched_from_bundle() -> Result<bool, BundleError> {
    match std::env::current_exe() {
        Ok(path) => {
            // Überprüfe, ob die aktuelle Executable INNERHALB eines Bundles liegt,
            // indem in der Verzeichnisstruktur nach einem Bundle-Root gesucht wird.
            Ok(find_bundle_root(&path).is_some())
        },
        // Falls der Programmpfad nicht ermittelt werden kann, gib einen passenden IO-Fehler zurück.
        Err(e) => Err(BundleError::IoError(e)),
    }
}
