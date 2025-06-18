use std::path::{Path, PathBuf};
use crate::is_bundle_at_path; // Funktion, die prüft, ob ein bestimmter Pfad ein Bundle ist

/// Findet das Wurzelverzeichnis (Root) eines Bundles, ausgehend vom gegebenen Ausführungspfad (`exe_path`).
/// Gibt den äußersten Bundle-Pfad (also das äußerste gefundene Bundle) als Option zurück.
/// Falls kein Bundle gefunden wurde, wird None zurückgegeben.
pub fn find_bundle_root(exe_path: &Path) -> Option<PathBuf> {
    // Starte mit dem übergeordneten Verzeichnis des exe_path
    let mut current_dir = exe_path.parent()?;
    // Vektor zum Speichern aller gefundenen Bundle-Verzeichnisse in der Hierarchie
    let mut found_bundles = Vec::new();

    // Durchlaufe die Verzeichnishierarchie nach oben
    loop {
        // Prüfe, ob das aktuelle Verzeichnis ein Bundle ist
        if is_bundle_at_path(current_dir) {
            // Falls ja, füge den Pfad zur Liste der gefundenen Bundles hinzu
            found_bundles.push(current_dir.to_path_buf());
        }

        // Setze current_dir auf das übergeordnete Verzeichnis
        current_dir = current_dir.parent()?;

        // Stoppe die Schleife, wenn das Root-Verzeichnis erreicht wurde
        if current_dir == Path::new("/") {
            break;
        }
    }

    // Gib das äußerste (letzte gefundene) Bundle zurück (oder None, falls keins gefunden)
    found_bundles.last().cloned()
}
