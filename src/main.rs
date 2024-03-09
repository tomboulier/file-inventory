// Importation des bibliothèques nécessaires
use std::env;
use std::fs::{self};
use std::path::Path;

// Documentation de la fonction principale
/// Main function to count `.nii.gz` files in given directory and its subdirectories.
fn main() {
    // Récupération du dossier à analyser à partir des arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_directory>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];

    // Appel de la fonction pour compter les fichiers
    match count_files(path) {
        Ok(count) => println!("Total `.nii.gz` files found: {}", count),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// Documentation pour la fonction de comptage de fichiers
/// Recursively counts `.nii.gz` files in the specified directory and its subdirectories.
/// Returns the total count of `.nii.gz` files found.
fn count_files<P: AsRef<Path>>(path: P) -> Result<usize, std::io::Error> {
    let mut count = 0;
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Récursivité pour les sous-dossiers
            count += count_files(&path)?;
        } else if let Some(ext) = path.extension() {
            // Compte seulement les fichiers avec l'extension `.nii.gz`
            if ext == "gz" && path.file_stem().unwrap_or_default().to_str().unwrap_or_default().ends_with(".nii") {
                count += 1;
            }
        }
    }

    Ok(count)
}
