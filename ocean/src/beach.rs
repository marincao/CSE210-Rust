use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    crabs: Vec<Crab>,
    clans: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        Beach {crabs: Vec::new(), clans: ClanSystem::new()}
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crabs.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crabs.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        if self.crabs.is_empty(){
            return None;
        } else{
            return self.crabs.iter().max_by_key(|crab| crab.speed());
        }
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut crabs_vec = Vec::new();
        for crab in self.crabs.iter() {
            if crab.name() == name{
                crabs_vec.push(crab);
            }
        }
        return crabs_vec;
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        let color = Color::cross(&self.crabs[i].color(), &self.crabs[j].color());
        let speed = 1;
        let diet = Diet::random_diet();
        let crab = Crab::new(name, speed, color, diet);
        self.add_crab(crab);
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.clans
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        self.clans.add_member_to_clan(clan_id, crab_name);
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        if !self.clans.clans.contains_key(id1) || !self.clans.clans.contains_key(id2){
            return Err(String::from("empty_clan!"));
        }
        let clan1 = self.clans.get_clan_member_names(id1);
		let clan2 = self.clans.get_clan_member_names(id2);
        let mut speed1 = 0;
        let mut speed2 = 0;
	    for (index, name) in clan1.iter().enumerate(){
            let crab = self.find_crabs_by_name(name)[0];
            speed1 += crab.speed();
        }
        for (index, name) in clan2.iter().enumerate(){
            let crab = self.find_crabs_by_name(name)[0];
            speed2 += crab.speed();
        }
	    speed1 = speed1/(clan1.len() as u32);
        speed2 = speed2/(clan2.len() as u32);
		if speed1 > speed2 {
			Ok(Some(id1.to_string().clone()))
		} else if speed1 < speed2 {
			Ok(Some(id2.to_string().clone()))
		} else {
			Ok(None)
		}
    }
}
