#![allow(dead_code)]
#![allow(unused_variables)]

use std::hash::{Hash, Hasher};

use advent_of_code::prelude::*;
use ahash::AHasher;

#[derive(Clone, Hash)]
pub struct Resources {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
}

impl Resources {
    pub fn new(ore: u64, clay: u64, obsidian: u64) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode: 0,
        }
    }

    fn can_build(&self, blueprint: &Resources) -> Option<Self> {
        let Some(ore) = self.ore.checked_sub(blueprint.ore)
        else { return None };
        let Some(clay) = self.clay.checked_sub(blueprint.clay)
        else { return None };
        let Some(obsidian) = self.obsidian.checked_sub(blueprint.obsidian)
        else { return None };
        let Some(geode) = self.geode.checked_sub(blueprint.geode)
        else { return None };
        Some(Self {
            ore,
            clay,
            obsidian,
            geode,
        })
    }
}

#[derive(Clone, Hash)]
struct Pack {
    resources: Resources,

    ore_robots: u64,
    clay_robots: u64,
    obsidian_robots: u64,
    geode_robots: u64,
}

impl Pack {
    fn gather(&mut self) {
        self.resources.ore += self.ore_robots;
        self.resources.clay += self.clay_robots;
        self.resources.obsidian += self.obsidian_robots;
        self.resources.geode += self.geode_robots;
    }

    fn build_ore_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        self.resources.can_build(&blueprint.ore).map(|resources| {
            let mut pack = self.clone();
            pack.resources = resources;
            pack
        })
    }

    fn build_clay_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        self.resources.can_build(&blueprint.clay).map(|resources| {
            let mut pack = self.clone();
            pack.resources = resources;
            pack
        })
    }

    fn build_obsidian_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        self.resources
            .can_build(&blueprint.obsidian)
            .map(|resources| {
                let mut pack = self.clone();
                pack.resources = resources;
                pack
            })
    }

    fn build_geode_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        self.resources.can_build(&blueprint.geode).map(|resources| {
            let mut pack = self.clone();
            pack.resources = resources;
            pack
        })
    }
}

impl Default for Pack {
    fn default() -> Self {
        Self {
            resources: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

pub struct Blueprint {
    ore: Resources,
    clay: Resources,
    obsidian: Resources,
    geode: Resources,
}

impl Blueprint {
    fn max_geodes(&self, max_time: u64) -> u64 {
        let mut memo = HashMap::new();
        self.max_geodes_rec_memoized(
            &mut memo,
            Pack::default(),
            0,
            max_time,
            false,
            false,
            false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn max_geodes_rec_memoized(
        &self,
        memo: &mut HashMap<u64, u64>,
        pack: Pack,
        time: u64,
        max_time: u64,
        skipped_ore: bool,
        skipped_clay: bool,
        skipped_obsidian: bool,
    ) -> u64 {
        let mut hasher = AHasher::default();
        pack.hash(&mut hasher);
        time.hash(&mut hasher);
        let hash = hasher.finish();
        if let Some(max) = memo.get(&hash) {
            return *max;
        }
        let max = self.max_geodes_rec(
            memo,
            pack,
            time,
            max_time,
            skipped_ore,
            skipped_clay,
            skipped_obsidian,
        );
        memo.insert(hash, max);
        max
    }

    #[allow(clippy::too_many_arguments)]
    fn max_geodes_rec(
        &self,
        memo: &mut HashMap<u64, u64>,
        mut pack: Pack,
        time: u64,
        max_time: u64,
        skipped_ore: bool,
        skipped_clay: bool,
        skipped_obsidian: bool,
    ) -> u64 {
        if time >= max_time {
            return pack.resources.geode;
        }

        if let Some(mut pack) = pack.build_geode_robot(self) {
            pack.gather();
            pack.geode_robots += 1;
            return self.max_geodes_rec_memoized(
                memo,
                pack,
                time + 1,
                max_time,
                false,
                false,
                false,
            );
        }

        let mut max = 0;

        let mut can_build_ore = false;
        let mut can_build_clay = false;
        let mut can_build_obsidian = false;
        if skipped_ore {
            can_build_ore = true;
        } else if let Some(mut pack) = pack.build_ore_robot(self) {
            pack.gather();
            pack.ore_robots += 1;
            let next = self.max_geodes_rec_memoized(
                memo,
                pack,
                time + 1,
                max_time,
                false,
                false,
                false,
            );
            if next > max {
                max = next;
            }
            can_build_ore = true;
        }
        if skipped_clay {
            can_build_clay = true;
        } else if let Some(mut pack) = pack.build_clay_robot(self) {
            pack.gather();
            pack.clay_robots += 1;
            let next = self.max_geodes_rec_memoized(
                memo,
                pack,
                time + 1,
                max_time,
                false,
                false,
                false,
            );
            if next > max {
                max = next;
            }
            can_build_clay = true;
        }
        if skipped_obsidian {
            can_build_obsidian = true;
        } else if let Some(mut pack) = pack.build_obsidian_robot(self) {
            pack.gather();
            pack.obsidian_robots += 1;
            let next = self.max_geodes_rec_memoized(
                memo,
                pack,
                time + 1,
                max_time,
                false,
                false,
                false,
            );
            if next > max {
                max = next;
            }
            can_build_obsidian = true;
        }

        pack.gather();
        let next = self.max_geodes_rec_memoized(
            memo,
            pack,
            time + 1,
            max_time,
            can_build_ore,
            can_build_clay,
            can_build_obsidian,
        );
        if next > max {
            max = next;
        }

        max
    }
}

impl std::str::FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = regex_captures!(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.", s).unwrap();
        let ore = cap[1].parse()?;
        let clay = cap[2].parse()?;
        let obsidian_ore = cap[3].parse()?;
        let obsidian_clay = cap[4].parse()?;
        let geode_ore = cap[5].parse()?;
        let geode_obsidian = cap[6].parse()?;
        Ok(Blueprint {
            ore: Resources::new(ore, 0, 0),
            clay: Resources::new(clay, 0, 0),
            obsidian: Resources::new(obsidian_ore, obsidian_clay, 0),
            geode: Resources::new(geode_ore, 0, geode_obsidian),
        })
    }
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = Blueprint>> {
    Ok(parse::raw_lines(fh).map(|s| s.parse().unwrap()))
}

pub fn part1(blueprints: impl Iterator<Item = Blueprint>) -> Result<u64> {
    let mut total = 0;
    for (i, blueprint) in blueprints.enumerate() {
        total += blueprint.max_geodes(24) * (i as u64 + 1);
    }
    Ok(total)
}

pub fn part2(mut blueprints: impl Iterator<Item = Blueprint>) -> Result<u64> {
    Ok(blueprints.by_ref().next().unwrap().max_geodes(32)
        * blueprints.by_ref().next().unwrap().max_geodes(32)
        * blueprints.by_ref().next().unwrap().max_geodes(32))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 19).unwrap()).unwrap()).unwrap(),
        1147
    );
    assert_eq!(
        part2(parse(parse::data(2022, 19).unwrap()).unwrap()).unwrap(),
        3080
    );
}
