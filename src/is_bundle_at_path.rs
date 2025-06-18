use std::path::Path;
use crate::{
    is_app_bundle_dir,         // Prüft, ob ein Pfad ein App-Bundle ist
    is_framework_bundle_dir,   // Prüft, ob ein Pfad ein Framework-Bundle ist
    is_service_bundle_dir,     // Prüft, ob ein Pfad ein Service-Bundle ist
    is_toolset_bundle_dir      // Prüft, ob ein Pfad ein Toolset-Bundle ist
};

/// Prüft, ob der übergebene Pfad irgendeinen gültigen Bundle-Typ repräsentiert.
/// Es werden alle bekannten Bundle-Typen abgefragt (App, Toolset, Service, Framework).
///
/// Gibt `true` zurück, wenn mindestens einer der Bundle-Typen auf den Pfad zutrifft,
/// ansonsten `false`.
pub fn is_bundle_at_path(path: &Path) -> bool {
    // Gibt true zurück, sobald einer der Typ-Checks positiv ist (OR-Verknüpfung)
    is_app_bundle_dir(path)
        || is_toolset_bundle_dir(path)
        || is_service_bundle_dir(path)
        || is_framework_bundle_dir(path)
}
