use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
	pub crabs: Vec<Crab>,
	pub clansystem: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        Beach { crabs: Vec::new(), clansystem: ClanSystem::new() }
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
		let mut highest_speed = 0;
		let mut fastest_crab: Option<&Crab> = None;
		for crab in &self.crabs {
			if crab.speed() > highest_speed {
				fastest_crab = Some(&crab);	
				highest_speed = crab.speed;
			}
		}
		fastest_crab
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut crabs_with_name = Vec::new();
		for crab in &self.crabs {
			if crab.name() == name {
				crabs_with_name.push(crab);
			}
		}
		crabs_with_name
    }
	
	pub fn breed(&self, i: usize, j: usize, name: String) -> Crab {
		let speed = 1;
		let color = Color::cross(self.get_crab(i).color(), self.get_crab(j).color());
		let diet = Diet::random_diet();
		Crab::new(name, speed, color, diet)
	}

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        if i >= self.crabs.len() || j >= self.crabs.len() {
			panic!("i or j out of the range!");
		}
		self.crabs.push(self.breed(i, j, name));
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.clansystem
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
		self.clansystem.add_member_to_clan(clan_id, crab_name);
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
		let option1 = self.clansystem.clans.get(id1); 
		let option2 = self.clansystem.clans.get(id2); 
		if !option1.is_none() && !option2.is_none() {
			let names1 = &self.clansystem.get_clan_member_names(id1);
			let names2 = &self.clansystem.get_clan_member_names(id2);
			let average1 = self.average_speed(names1);
			let average2 = self.average_speed(names2);
			if average1 > average2 {
				Ok(Some(id1.to_string().clone()))
			} else if average1 < average2 {
				Ok(Some(id2.to_string().clone()))
			} else {
				Ok(None)
			}
		} else {
			Err(String::from("invalid id!"))
		}
    }


	pub fn average_speed(&self, names: &Vec<String>) -> f64 {
		let mut sum = 0;
		for name in names {
			for crab in &self.find_crabs_by_name(name) {
				sum += crab.speed();
			}
		}
		sum as f64 / names.len() as f64
	}
}	
