pub use lru_cache::Cache; 

#[cfg(test)] 
mod tests {
    use lru_cache::{cache::CacheOperations, file_storage::CacheStorage};

    use super::*;

    // Test pour vérifier l'ajout et la récupération d'éléments dans le cache
    #[test]
    fn test_cache_ajouter_et_recup() {
        let mut cache = Cache::nouveau(2);
        cache.ajouter("cle test 1".to_string(), "valeur test 1".to_string());
        cache.ajouter("cle test 2".to_string(), "valeur test 2".to_string());

        assert_eq!(cache.obtenir(&"cle test 1".to_string()), Some(&"valeur test 1".to_string()));
        assert_eq!(cache.obtenir(&"cle test 2".to_string()), Some(&"valeur test 2".to_string()));
    }

    // Test pour vérifier que l'élément le plus ancien est supprimé
    #[test]
    fn test_suppression_ancien() {
        let mut cache = Cache::nouveau(2);
        cache.ajouter("cle test 1".to_string(), "valeur test 1".to_string());
        cache.ajouter("cle test 2".to_string(), "valeur test 2".to_string());
        cache.ajouter("cle test 3".to_string(), "valeur test 3".to_string());

        assert_eq!(cache.obtenir(&"cle test 1".to_string()), None);
        assert_eq!(cache.obtenir(&"cle test 2".to_string()), Some(&"valeur test 2".to_string()));
        assert_eq!(cache.obtenir(&"cle test 3".to_string()), Some(&"valeur test 3".to_string()));
    }

    // Test pour sauvegarder et charger le cache depuis un fichier
    #[test]
    fn test_cache_persistent_save_et_charg() {
        let mut cache = Cache::nouveau(2);
        cache.ajouter("cle test 1".to_string(), "valeur test 1".to_string());
        cache.ajouter("cle test 2".to_string(), "valeur test 2".to_string());
        cache.sauvegarder("test_cache.txt");

        let mut loaded_cache = Cache::<String, String>::charger_persistant(2, "test_cache.txt");

        assert_eq!(loaded_cache.obtenir(&"cle test 1".to_string()), Some(&"valeur test 1".to_string()));
        assert_eq!(loaded_cache.obtenir(&"cle test 2".to_string()), Some(&"valeur test 2".to_string()));
    }

    // Test pour vérifier le format invalide du fichier
    #[test]
    fn test_cache_persistent_format_pas_valide() {
        let _cache: Cache<String, String> = Cache::nouveau(2);
        std::fs::write("invalid_test_cache.txt", "cle_invalide\n").unwrap();

        let mut loaded_cache = Cache::<String, String>::charger_persistant(2, "invalid_test_cache.txt");

        assert_eq!(loaded_cache.obtenir(&"invalide_cle_test".to_string()), None);
    }
}

