use std::path::PathBuf;
use once_cell::sync::OnceCell;

use crate::{
    {find_bundle_root, BundleError}, // Importiere die Funktion zum Finden des Bundle-Roots und den Fehler-Typ
                   // Funktion, um den aktuellen Pfad des Programms zu bestimmen
};

static BUNDLE_PATH: OnceCell<Result<Option<PathBuf>, BundleError>> = OnceCell::new();

/// Ermittelt den Pfad des aktuellen Bundles (sofern vorhanden).
/// Gibt bei Erfolg einen Option<PathBuf> zurück:
/// - Some(pfad) falls ein Bundle gefunden wurde,
/// - None falls kein Bundle gefunden wurde.
/// Gibt bei Fehlern ein passendes BundleError zurück.
pub fn get_current_launched_bundle_path() -> &'static Result<Option<PathBuf>, BundleError> {
    BUNDLE_PATH.get_or_init(|| {
        let exe_path = std::env::current_exe()?;
        Ok(find_bundle_root(&exe_path))
    })
}
