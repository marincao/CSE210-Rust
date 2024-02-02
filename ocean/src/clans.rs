use std::{collections::HashMap};

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
        match self.clans.get(clan_id) {
			None => Vec::new(),
			Some(_clan) => self.clans.get(clan_id).unwrap().to_vec(),
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
        match self.clans.get(clan_id) {
			None => 0,
			Some(clan) => clan.len(),
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
        index
    }

    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        if let Some(clan) = self.clans.get_mut(clan_id) {
			clan.push(String::from(crab_name));
		} else {
			self.clans.insert(String::from(clan_id).clone(), vec![String::from(crab_name)]);
		}
    }
}