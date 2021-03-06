#[cfg(test)]
mod tests {
    #[cfg(feature = "non_solution_test")]
    use crate::day06::naive::fish::Fish;

    use crate::day06::{naive::world::World, test::INPUT};

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn simulates_example_problem_fish_count() {
        let initial_fish = vec![
            Fish::try_from(3).unwrap(),
            Fish::try_from(4).unwrap(),
            Fish::try_from(3).unwrap(),
            Fish::try_from(1).unwrap(),
            Fish::try_from(2).unwrap(),
        ];
        let mut world: World = initial_fish.try_into().unwrap();
        for _ in 0..18 {
            let new_fish = world.day();
            world.add_fish(new_fish);
        }
        assert_eq!(26, world.how_many_fish());
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn simulates_example_problem() {
        let initial_fish = vec![
            Fish::try_from(3).unwrap(),
            Fish::try_from(4).unwrap(),
            Fish::try_from(3).unwrap(),
            Fish::try_from(1).unwrap(),
            Fish::try_from(2).unwrap(),
        ];
        let mut world: World = initial_fish.try_into().unwrap();
        let mut result = String::new();
        for _ in 0..=18 {
            result.push_str(&format!("{}", world));
            let new_fish = world.day();
            world.add_fish(new_fish);
        }
        let expected = r##"Initial state: 3,4,3,1,2
After  1 day:  2,3,2,0,1
After  2 days: 1,2,1,6,0,8
After  3 days: 0,1,0,5,6,7,8
After  4 days: 6,0,6,4,5,6,7,8,8
After  5 days: 5,6,5,3,4,5,6,7,7,8
After  6 days: 4,5,4,2,3,4,5,6,6,7
After  7 days: 3,4,3,1,2,3,4,5,5,6
After  8 days: 2,3,2,0,1,2,3,4,4,5
After  9 days: 1,2,1,6,0,1,2,3,3,4,8
After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8
"##;
        assert_eq!(expected, result);
    }

    #[test]
    fn computes_result() {
        let mut world = World::try_from(INPUT).unwrap();
        for _ in 0..80 {
            let new_fish = world.day();
            world.add_fish(new_fish);
        }
        assert_eq!(372300, world.how_many_fish());
    }
}
