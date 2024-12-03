# LRU Cache in Rust

🇬🇧 **English Version**

This project implements a **Least Recently Used (LRU) cache** in Rust, with no external dependencies. The cache automatically deletes the least recently used item when the cache exceeds its capacity, providing an efficient way to store and retrieve key-value pairs.

## Features
- **LRU Deletion**: Automatically deletes the least recently used item when the cache reaches its capacity.
- **Persistence**: Saves the cache to a text file and loads it back, allowing persistent storage between program runs.
- **Testing**: Includes unit and integration tests to ensure the cache operates correctly (tests/integration_tests.rs).

## Project Structure
- **src/cache.rs**: Contains the `Cache` struct and its methods.
- **src/lib.rs**: Exposes the `Cache` struct to other modules or projects.
- **src/main.rs**: Demonstrates the usage of the `Cache` struct with examples.
- **tests/integration_tests.rs**: Contains integration tests that verify cache functionality such as adding items, deletion behavior, and persistence.

## Running Tests

To run the tests, use the following command:

```bash
cargo test
```

## Available Methods

- **Cache::nouveau(capacity)**: Creates a new cache with the specified capacity.
- **Cache::ajouter(key, value)**: Adds a key-value pair to the cache.
- **Cache::obtenir(key)**: Retrieves the value for the specified key.
- **Cache::sauvegarder(filename)**: Saves the cache to a txt file.
- **Cache::charger_persistant(capacity, filename)**: Loads a cache from a txt file.

---

🇫🇷 **Version Française**

Ce projet implémente un **cache LRU (Least Recently Used)** en Rust, sans dépendances externes. Le cache supprime automatiquement l'élément le moins récemment utilisé lorsque la capacité du cache est atteinte, offrant ainsi un moyen efficace de stocker et récupérer des paires clé-valeur.

## Fonctionnalités
- **Suppression LRU** : Supprime automatiquement l'élément le moins récemment utilisé lorsque le cache atteint sa capacité.
- **Persistance** : Sauvegarde le cache dans un fichier texte et le charge à nouveau, permettant un stockage persistant entre les exécutions du programme.
- **Tests** : Inclut des tests unitaires et d'intégration pour vérifier que le cache fonctionne correctement (tests/integration_tests.rs).

## Structure du projet
- **src/cache.rs** : Contient la structure `Cache` et ses méthodes.
- **src/lib.rs** : Expose la structure `Cache` aux autres modules ou projets.
- **src/main.rs** : Montre l'utilisation de la structure `Cache` avec des exemples.
- **tests/integration_tests.rs** : Contient des tests d'intégration qui vérifient le bon fonctionnement du cache, tels que l'ajout d'éléments, le comportement de suppression et la persistance.

## Exécution des tests

Pour exécuter les tests, utilisez la commande suivante :

```bash
cargo test
```

## Exemple d'utilisation

Voici un exemple de la façon d'utiliser le programme principal pour ajouter des données dans le cache et observer si l'élément le plus ancien est supprimé lorsque la capacité est dépassée.

### main.rs

```rust
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
```
résultat : 
```
cle test 4:valeur test 4
cle test 3:valeur test 3
cle test 2:valeur test 2
```

- **Création du cache** : La méthode `Cache::nouveau(3)` crée un cache avec une capacité de 3 éléments.
- **Ajout d'éléments** : La méthode `ajouter()` permet d'ajouter des paires clé-valeur dans le cache.
- **Suppression automatique** : Lorsque le cache atteint sa capacité maximale, l'élément le plus ancien est supprimé (ici 3 donc 3 valeurs max !).
- **Sauvegarde et chargement persistant** : La méthode `sauvegarder()` permet de sauvegarder le cache dans un fichier, et `charger_persistant()` permet de le recharger, garantissant que les données sont conservées entre les sessions.

## Méthodes Disponibles

- **Cache::nouveau(capacité)** : Crée un nouveau cache avec la capacité spécifiée.
- **Cache::ajouter(clé, valeur)** : Ajoute une paire clé-valeur au cache.
- **Cache::obtenir(clé)** : Récupère la valeur pour la clé spécifiée.
- **Cache::sauvegarder(fichier)** : Sauvegarde le cache dans un fichier texte.
- **Cache::charger_persistant(capacité, fichier)** : Charge un cache à partir d'un fichier texte.
