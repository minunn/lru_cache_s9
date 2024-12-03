// J'importe le module cache et j'importe la structure Cache dans le reste du projet.
use lru_cache::Cache;

fn main() {
    // Je crée une cache mutable de type Cache<String, String> avec une capacité de 3
    let mut cache: Cache<String, String> = Cache::nouveau(3);

    // J'ajoute des clés et des valeurs dans le cache pour les stocker
    cache.ajouter("cle test 1".to_string(), "valeur test 1".to_string());
    cache.ajouter("cle test 2".to_string(), "valeur test 2".to_string());
    cache.ajouter("cle test 3".to_string(), "valeur test 3".to_string());

    // J'affiche les valeurs du cache que je viens de stocker
    println!("Valeur du cache après les insertions: {:?}", cache);
    
    // J'ajoute une nouvelle clé et valeur dans le cache pour voir si l'élément le plus ancien est supprimé
    cache.ajouter("cle test 4".to_string(), "valeur test 4".to_string());
    println!("Valeur du cache après l'ajout de cle test 4: {:?}", cache);

    // Pour finir, je sauvegarde le cache dans un fichier texte pour voir
    // si les valeurs sont bien stockées
    cache.sauvegarder("mon_cache.txt");
}