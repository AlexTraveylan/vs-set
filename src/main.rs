use std::fs;
use std::path::Path;
use std::env;
use vs_set::factory::StrategyFactory;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage: {} <python|typescript>", args[0]);
        return;
    }

    let language = &args[1];
    let vscode_dir = Path::new(".vscode");

    // Création du dossier .vscode s'il n'existe pas
    if !vscode_dir.exists() {
        fs::create_dir_all(vscode_dir).expect("Impossible de créer le dossier .vscode");
    }

    // Utilisation du factory pattern pour obtenir la stratégie appropriée
    let strategy = StrategyFactory::create_strategy(language);
    
    // Copie des fichiers en utilisant la stratégie
    match strategy.copy_files(vscode_dir) {
        Ok(_) => println!("Configuration {} copiée avec succès!", language),
        Err(e) => println!("Erreur lors de la copie: {}", e)
    }
}
