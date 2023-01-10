use rand::{seq::SliceRandom, Rng};

#[derive(Debug)]
pub struct Names {
    pub male_names: Vec<&'static str>,
    pub female_names: Vec<&'static str>,
    pub names: Vec<&'static str>,
}

impl Names {
    pub fn load() -> Self {
        let mut male_names = Vec::with_capacity(259);
        let mut female_names = Vec::with_capacity(199);
        let mut names = Vec::with_capacity(458);
        for row in include_str!("../../assets/data/names.txt").lines() {
            let mut split = row.split(',');
            let name = split.next().unwrap();
            if name.is_empty() {
                continue;
            }
            let sex = split.next().expect(name);
            if sex == "1" {
                male_names.push(name);
            } else {
                female_names.push(name);
            }
            names.push(name);
        }

        Self {
            male_names,
            female_names,
            names,
        }
    }

    pub fn random_name<R: Rng + ?Sized>(&self, rng: &mut R) -> &str {
        self.names.choose(rng).unwrap()
    }

    pub fn random_male_name<R: Rng + ?Sized>(&self, rng: &mut R) -> &str {
        self.male_names.choose(rng).unwrap()
    }

    pub fn random_female_name<R: Rng + ?Sized>(&self, rng: &mut R) -> &str {
        self.female_names.choose(rng).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Names;

    #[test]
    fn names_load() {
        let names = Names::load();
        assert!(names.male_names.len() > 0);
        assert!(names.female_names.len() > 0);
        assert!(names.names.len() > 0);
    }
}
