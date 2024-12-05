use super::cache::{Cache, CacheOperations};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::str::FromStr;
use std::hash::Hash;
use std::fmt::Debug;

pub trait CacheStorage<K, V> {
    fn sauvegarder(&self, fichier: &str);
    fn charger_persistant(capacite: usize, fichier: &str) -> Self;
}

impl<K, V> CacheStorage<K, V> for Cache<K, V>
where
    K: ToString + FromStr + Clone + Hash + Eq + Debug,
    V: ToString + FromStr + Clone + Debug,
{
    // Je crée une méthode sauvegarder qui prend en paramètre le nom du fichier où sauvegarder le cache
     fn sauvegarder(&self, fichier: &str) {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(fichier)
            .expect("Impossible d'ouvrir le fichier pour écriture.");

            // J'écris chaque clé et valeur dans le fichier texte pour sauvegarder le cache actuel
        for cle in &self.ordre {
            if let Some(valeur) = self.map.get(cle) {
                writeln!(f, "{}:{}", cle.to_string(), valeur.to_string())
                    .expect("Erreur lors de l'écriture dans le fichier.");
            }
        }
    }

    // Je crée une méthode charger_persistant qui prend en paramètre la capacité du cache et le nom du fichier
     fn charger_persistant(capacite: usize, fichier: &str) -> Self {
        let mut cache = Cache::nouveau(capacite);
        let mut contenu = String::new();

        OpenOptions::new()
            .read(true)
            .open(fichier)
            .expect("Impossible d'ouvrir le fichier pour lecture.")
            .read_to_string(&mut contenu)
            .expect("Erreur lors de la lecture du fichier.");

            // Je parcours chaque ligne du fichier pour ajouter les clés et valeurs dans le cache
        for ligne in contenu.lines() {
            let mut parts = ligne.splitn(2, ':');
            if let (Some(cle), Some(valeur)) = (parts.next(), parts.next()) {
                if let (Ok(cle_parse), Ok(valeur_parse)) = (cle.parse::<K>(), valeur.parse::<V>()) {
                    cache.ajouter(cle_parse, valeur_parse);
                }
            }
        }
        cache
    }
}


// Test unitaires pour la sauvegarde et le chargement du cache

#[cfg(test)]
mod tests {
    use super::*;

    // Ici je teste la sauvegarde et le chargement du cache
    #[test]
    fn test_sauvegarde_et_chargement() {
        let mut cache = Cache::nouveau(2);
        cache.ajouter("valeur a".to_string(), "1".to_string());
        cache.sauvegarder("test.txt");
        let mut cache_charge = Cache::<String, String>::charger_persistant(2, "test.txt");
        assert_eq!(cache_charge.obtenir(&"valeur a".to_string()), Some(&"1".to_string()));
    }
}