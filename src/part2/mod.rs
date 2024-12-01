use std::collections::{HashSet, VecDeque};
use crate::cloudmap::Coordinate;
use crate::part1::part1;

#[test]
pub fn part2() {
    let cloud_map = part1().0;
    let cloud_particles = cloud_map.iter()
        .filter(|(_, cloud)| **cloud > 0)
        .map(|cc| {
            Coordinate::from(*cc.0)
        })
        .collect::<Vec<_>>();

    let mut unvisited: HashSet<Coordinate> = cloud_particles.into_iter().collect();
    let mut groups = Vec::new();

    while let Some(&start) = unvisited.iter().next() {
        let mut group = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            if unvisited.remove(&current) {
                group.insert(current);
                for neighbor in &unvisited {
                    if neighbor.shares_side(&current) {
                        queue.push_back(*neighbor);
                    }
                }
            }
        }

        groups.push(group);
    }

    println!("{:?}", groups.len());
}