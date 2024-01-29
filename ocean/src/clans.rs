use std::{borrow::Borrow, collections::HashMap, mem};

#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    pub clans : HashMap<String, Vec<String>>
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem {clans: HashMap::new()}
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        if self.clans.contains_key(clan_id){
            return self.clans.get(clan_id).unwrap().to_vec();
        } else{
            return Vec::new();
        }
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        if self.clans.contains_key(clan_id){
            return self.clans.get(clan_id).unwrap().len();
        } else{
            return 0;
        }
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        let mut largest = 0;
        let mut index = None;
        for (key, value) in self.clans.iter() {
            if value.len() > largest {
                largest = value.len();
                index = Some(key.clone());
            }
        }
        return index;
    }

    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        if let Some(clan) = self.clans.get_mut(clan_id) {
			clan.push(String::from(crab_name));
		} else {
            let mut list: Vec<String> = Vec::new();
            list.push(String::from(crab_name));
			self.clans.insert(String::from(clan_id).clone(), list);
		}
    }
}