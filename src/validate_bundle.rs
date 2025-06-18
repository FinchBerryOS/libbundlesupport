use std::{fs, path::Path};
use goblin::elf;
use crate::{get_current_launched_bundle_path, get_loaded_bundle_info_config, BundleValidationError, BundleValidationResult};

#[derive(Debug)]
pub enum Architecture {
    X86,      // i386
    X86_64,   // x86_64
    ARM64,    // AArch64
    ARM,      // ARM 32-bit
    Other(u16),
}

pub fn detect_elf_architecture<P: AsRef<Path>>(path: P) -> Result<Architecture, Box<dyn std::error::Error>> {
    let data = fs::read(path)?;
    let elf = elf::Elf::parse(&data)?;
    
    let arch = match elf.header.e_machine {
        elf::header::EM_386 => Architecture::X86,
        elf::header::EM_X86_64 => Architecture::X86_64,
        elf::header::EM_AARCH64 => Architecture::ARM64,
        elf::header::EM_ARM => Architecture::ARM,
        other => Architecture::Other(other),
    };
    
    Ok(arch)
}

pub fn is_x86_64<P: AsRef<Path>>(path: P) -> Result<bool, Box<dyn std::error::Error>> {
    match detect_elf_architecture(path)? {
        Architecture::X86_64 => Ok(true),
        _ => Ok(false),
    }
}

pub fn is_arm64<P: AsRef<Path>>(path: P) -> Result<bool, Box<dyn std::error::Error>> {
    match detect_elf_architecture(path)? {
        Architecture::ARM64 => Ok(true),
        _ => Ok(false),
    }
}

pub fn validate_bundle() -> Result<BundleValidationResult, BundleValidationError> {
    // Es wird versucht den Aktuellen Bundle Path zu ermitteln
    let bundle_path = match get_current_launched_bundle_path() {
        Ok(Some(path)) => path,
        Ok(None) => {
            eprintln!("Kein Bundle-Pfad in der Config gefunden");
            return Err(BundleValidationError::ConfigLoadError("Kein Bundle-Pfad in der Config gefunden".to_string()));
        }
        Err(e) => {
            return Err(BundleValidationError::ConfigLoadError(format!("{:?}", e)));
        }
    };

    // Es wird versucht die Aktuelle Konfiguration zu laden
    let bunlde_config = match get_loaded_bundle_info_config() {
        Ok(config) => config,
        Err(e) => { return Err(BundleValidationError::ConfigLoadError(format!("{:?}", e))); }
    };

    // evtl. Warnungen sammeln
    let mut warnings = Vec::new();
    Ok(BundleValidationResult {
        is_valid: true,
        message: "Bundle ist gültig".to_string(),
        warnings,
    })
}