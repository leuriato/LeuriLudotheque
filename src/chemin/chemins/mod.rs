pub mod err;

use crate::chemin::chemins::err::*;

use std::path::PathBuf;

const PREFIX_XDG: &str = "ludotheque";

#[derive(Debug)]
pub enum XDG {
    CACHE,
    CONFIG,
    DATA,
    STATE,
}

fn obtenir_repertoires_xdg() -> Result<xdg::BaseDirectories, Erreur> {
    match xdg::BaseDirectories::with_prefix(PREFIX_XDG) {
        Ok(valeur) => Ok(valeur),
        Err(erreur) => Err(Erreur::ErreurXDG(ErreurXDG { erreur })),
    }
}

pub fn determiner_chemin(nom: String, xdg_type: XDG) -> Result<PathBuf, Erreur> {
    let xdg_dirs = obtenir_repertoires_xdg()?;

    let place_fichier = match xdg_type {
        XDG::CACHE => xdg_dirs.place_cache_file(nom.clone()),
        XDG::CONFIG => xdg_dirs.place_config_file(nom.clone()),
        XDG::DATA => xdg_dirs.place_data_file(nom.clone()),
        XDG::STATE => xdg_dirs.place_state_file(nom.clone()),
    };

    let chemin = match place_fichier {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurRepertoireXDG { xdg_type, erreur }.as_err(),
    };

    Ok(PathBuf::from(chemin))
}

pub fn trouver_chemin(nom: String, xdg_type: XDG) -> Result<PathBuf, Erreur> {
    let xdg_dirs = obtenir_repertoires_xdg()?;

    let trouve_fichier = match xdg_type {
        XDG::CACHE => xdg_dirs.find_cache_file(nom.clone()),
        XDG::CONFIG => xdg_dirs.find_config_file(nom.clone()),
        XDG::DATA => xdg_dirs.find_data_file(nom.clone()),
        XDG::STATE => xdg_dirs.find_state_file(nom.clone()),
    };

    let chemin = match trouve_fichier {
        Some(valeur) => valeur,
        None => return ErreurFichierIntrouvable { xdg_type, nom }.as_err(),
    };

    Ok(PathBuf::from(chemin))
}

