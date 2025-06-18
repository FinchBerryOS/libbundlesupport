use std::path::Path;
use crate::valid_bundle_structure; // Prüft, ob die Bundle-Verzeichnisstruktur korrekt ist

/// Prüft, ob der angegebene Pfad ein App-Bundle-Verzeichnis ist.
/// Bedingungen:
/// 1. Der Pfad zeigt auf ein Verzeichnis.
/// 2. Die Dateiendung ist ".appd".
/// 3. Die Bundle-Struktur ist gültig (Content/ und Content/Config.json existieren).
pub fn is_app_bundle_dir(path: &Path) -> bool {
    path.is_dir() 
        && path.extension().map_or(false, |ext| ext == "appd")
        && valid_bundle_structure(path)
}

/// Prüft, ob der angegebene Pfad ein Service-Bundle-Verzeichnis ist.
/// Bedingungen:
/// 1. Der Pfad zeigt auf ein Verzeichnis.
/// 2. Die Dateiendung ist ".serviced".
/// 3. Die Bundle-Struktur ist gültig.
pub fn is_service_bundle_dir(path: &Path) -> bool {
    path.is_dir() 
        && path.extension().map_or(false, |ext| ext == "serviced")
        && valid_bundle_structure(path)
}

/// Prüft, ob der angegebene Pfad ein Toolset-Bundle-Verzeichnis ist.
/// Bedingungen:
/// 1. Der Pfad zeigt auf ein Verzeichnis.
/// 2. Die Dateiendung ist ".toolsetd".
/// 3. Die Bundle-Struktur ist gültig.
pub fn is_toolset_bundle_dir(path: &Path) -> bool {
    path.is_dir() 
        && path.extension().map_or(false, |ext| ext == "toolsetd")
        && valid_bundle_structure(path)
}

/// Prüft, ob der angegebene Pfad ein Framework-Bundle-Verzeichnis ist.
/// Bedingungen:
/// 1. Der Pfad zeigt auf ein Verzeichnis.
/// 2. Die Dateiendung ist ".frameworkd".
/// 3. Die Bundle-Struktur ist gültig.
pub fn is_framework_bundle_dir(path: &Path) -> bool {
    path.is_dir() 
        && path.extension().map_or(false, |ext| ext == "frameworkd")
        && valid_bundle_structure(path)
}
