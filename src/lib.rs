//J'appel le module cache et j'importe la structure Cache dans le reste du projet.
//c'est ce qui permet d'utiliser la structure Cache dans le reste du projet en soit.
pub mod cache;
//Ici j'importe le module file_storage dans le reste du projet qui permet de sauvegarder et charger le cache.
pub mod file_storage;

//Ici j'importe le module cache et j'importe la structure Cache dans le reste du projet.
pub use cache::Cache;
