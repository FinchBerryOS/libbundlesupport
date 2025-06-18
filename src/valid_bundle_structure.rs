use std::path::Path;

/// Prüft, ob eine gültige Bundle-Struktur am angegebenen Pfad vorhanden ist.
/// Die Struktur gilt als gültig, wenn das Verzeichnis "Content/" existiert
/// und darin die Datei "Config.json" vorhanden ist.
pub fn valid_bundle_structure(bundle_path: &Path) -> bool {
    // Erstelle den Pfad zum "Content"-Verzeichnis innerhalb des Bundles
    let contents_dir = bundle_path.join("Content");
    // Erstelle den Pfad zur Datei "Config.json" im "Content"-Verzeichnis
    let config_file = contents_dir.join("Config.json");
    // Überprüfe, ob das "Content"-Verzeichnis existiert und ob darin die Datei "Config.json" liegt
    contents_dir.is_dir() && config_file.is_file()
}
