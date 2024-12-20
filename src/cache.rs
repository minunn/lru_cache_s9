use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::fmt::Debug;

pub trait CacheOperations<K, V> {
    fn ajouter(&mut self, cle: K, valeur: V);
    fn obtenir(&mut self, cle: &K) -> Option<&V>;
}

#[derive(Debug)]
pub struct Cache<K, V>
where
    K: Hash + Eq + Clone + Debug,
    V: Clone + Debug,
{
    capacite: usize,
    pub map: HashMap<K, V>,
    pub ordre: VecDeque<K>,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone + Debug,
    V: Clone + Debug,
{
    // Ici je crée une fonction nouveau qui prend en paramètre 
    // une capacité et qui retourne un cache
    pub fn nouveau(capacite: usize) -> Self {
        Cache {
            capacite,
            map: HashMap::new(),
            ordre: VecDeque::with_capacity(capacite),
        }
    }
}

impl<K, V> CacheOperations<K, V> for Cache<K, V>
where
    K: Hash + Eq + Clone + Debug,
    V: Clone + Debug,
{
    // Ici je crée une fonction obtenir qui prend en paramètre 
    // une clé et qui retourne la valeur associée à cette clé
    fn obtenir(&mut self, cle: &K) -> Option<&V> {
        if let Some(valeur) = self.map.get(cle) {
            self.ordre.retain(|k| k != cle);
            self.ordre.push_front(cle.clone());
            Some(valeur)
        } else {
            None
        }
    }

    // Ici je crée une fonction ajouter qui prend en paramètre 
    // une clé et une valeur et qui ajoute la clé et la valeur dans le cache
     fn ajouter(&mut self, cle: K, valeur: V) {
        if self.map.len() >= self.capacite {
            if let Some(plus_vieux) = self.ordre.pop_back() {
                self.map.remove(&plus_vieux);
            }
        }
        self.ordre.retain(|k| k != &cle);
        self.ordre.push_front(cle.clone());
        self.map.insert(cle, valeur);
    }
}

//tests unitaires pour l'ajout et l'obtention d'éléments dans le cache 
//et la suppression de l'élément le plus ancien

#[cfg(test)]
mod tests {
    use super::*;

    // Ici je teste l'ajout et l'obtention d'éléments dans le cache
    #[test]
    fn test_ajout_et_obtention() {
        let mut cache = Cache::nouveau(2);
        cache.ajouter("valeur a".to_string(), "1".to_string());
        cache.ajouter("valeur b".to_string(), "2".to_string());
        assert_eq!(cache.obtenir(&"valeur a".to_string()), Some(&"1".to_string()));
    }

    // Ici je teste la suppression de l'élément le plus ancien quand le cache est plein
    #[test]
    fn test_suppression_ancien() {
        let mut cache = Cache::nouveau(2);
        cache.ajouter("valeur a".to_string(), "1".to_string());
        cache.ajouter("valeur b".to_string(), "2".to_string());
        cache.ajouter("valeur c".to_string(), "3".to_string());
        assert_eq!(cache.obtenir(&"valeur a".to_string()), None);
        println!("{:?}", cache);
    }
}