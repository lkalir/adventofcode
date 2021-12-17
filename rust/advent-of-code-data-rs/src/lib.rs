use num_derive::FromPrimitive;

/// Input data for Advent of Code 2015
pub mod aoc_2015 {
    /// 2015 Day 1: Not Quite Lisp
    pub const DAY1: &str = concat!(include_str!("advent-of-code-data/2015/day1.txt"), "\0");
    /// 2015 Day 2: I Was Told There Would Be No Math
    pub const DAY2: &str = concat!(include_str!("advent-of-code-data/2015/day2.txt"), "\0");
    /// 2015 Day 3: Perfectly Spherical Houses in a Vacuum
    pub const DAY3: &str = concat!(include_str!("advent-of-code-data/2015/day3.txt"), "\0");
    /// 2015 Day 4: The Ideal Stocking Stuffer
    pub const DAY4: &str = concat!(include_str!("advent-of-code-data/2015/day4.txt"), "\0");
    /// 2015 Day 5: Doesn't He Have Intern-Elves For This?
    pub const DAY5: &str = concat!(include_str!("advent-of-code-data/2015/day5.txt"), "\0");
    /// 2015 Day 6: Probably a Fire Hazard
    pub const DAY6: &str = concat!(include_str!("advent-of-code-data/2015/day6.txt"), "\0");
    /// 2015 Day 7: Some Assembly Required
    pub const DAY7: &str = concat!(include_str!("advent-of-code-data/2015/day7.txt"), "\0");
    /// 2015 Day 8: Matchsticks
    pub const DAY8: &str = concat!(include_str!("advent-of-code-data/2015/day8.txt"), "\0");
    /// 2015 Day 9: All in a Single Night
    pub const DAY9: &str = concat!(include_str!("advent-of-code-data/2015/day9.txt"), "\0");
    /// 2015 Day 10: Elves Look, Elves Say
    pub const DAY10: &str = concat!(include_str!("advent-of-code-data/2015/day10.txt"), "\0");
    /// 2015 Day 11: Corporate Policy
    pub const DAY11: &str = concat!(include_str!("advent-of-code-data/2015/day11.txt"), "\0");
    /// 2015 Day 12: JSAbacusFramework.io
    pub const DAY12: &str = concat!(include_str!("advent-of-code-data/2015/day12.txt"), "\0");
    /// 2015 Day 13: Knights of the Dinner Table
    pub const DAY13: &str = concat!(include_str!("advent-of-code-data/2015/day13.txt"), "\0");
    /// 2015 Day 14: Reindeer Olympics
    pub const DAY14: &str = concat!(include_str!("advent-of-code-data/2015/day14.txt"), "\0");
    /// 2015 Day 15: Science for Hungry People
    pub const DAY15: &str = concat!(include_str!("advent-of-code-data/2015/day15.txt"), "\0");
    /// 2015 Day 16: Aunt Sue
    pub const DAY16: &str = concat!(include_str!("advent-of-code-data/2015/day16.txt"), "\0");
    /// 2015 Day 17: No Such Thing as Too Much
    pub const DAY17: &str = concat!(include_str!("advent-of-code-data/2015/day17.txt"), "\0");
    /// 2015 Day 18: Like a GIF For Your Yard
    pub const DAY18: &str = concat!(include_str!("advent-of-code-data/2015/day18.txt"), "\0");
    /// 2015 Day 19: Medicine for Rudolph
    pub const DAY19: &str = concat!(include_str!("advent-of-code-data/2015/day19.txt"), "\0");
    /// 2015 Day 20: Infinite Elves and Infinite Houses
    pub const DAY20: &str = concat!(include_str!("advent-of-code-data/2015/day20.txt"), "\0");
    /// 2015 Day 21: RPG Simulator 20XX
    pub const DAY21: &str = concat!(include_str!("advent-of-code-data/2015/day21.txt"), "\0");
    /// 2015 Day 22: Wizard Simulator 20XX
    pub const DAY22: &str = concat!(include_str!("advent-of-code-data/2015/day22.txt"), "\0");
    /// 2015 Day 23: Opening the Turing Lock
    pub const DAY23: &str = concat!(include_str!("advent-of-code-data/2015/day23.txt"), "\0");
    /// 2015 Day 24: It Hangs in the Balance
    pub const DAY24: &str = concat!(include_str!("advent-of-code-data/2015/day24.txt"), "\0");
    /// 2015 Day 25: Let It Snow
    pub const DAY25: &str = concat!(include_str!("advent-of-code-data/2015/day25.txt"), "\0");

    /// Array containing all inputs in chronological order
    pub const INPUTS: &[&str] = &[
        DAY1, DAY2, DAY3, DAY4, DAY5, DAY6, DAY7, DAY8, DAY9, DAY10, DAY11, DAY12, DAY13, DAY14,
        DAY15, DAY16, DAY17, DAY18, DAY19, DAY20, DAY21, DAY22, DAY23, DAY24, DAY25,
    ];
}

/// Input data for Advent of Code 2016
pub mod aoc_2016 {
    /// 2016 Day 1: No Time for a Taxicab
    pub const DAY1: &str = concat!(include_str!("advent-of-code-data/2016/day1.txt"), "\0");
    /// 2016 Day 2: Bathroom Security
    pub const DAY2: &str = concat!(include_str!("advent-of-code-data/2016/day2.txt"), "\0");
    /// 2016 Day 3: Squares With Three Sides
    pub const DAY3: &str = concat!(include_str!("advent-of-code-data/2016/day3.txt"), "\0");
    /// 2016 Day 4: Security Through Obscurity
    pub const DAY4: &str = concat!(include_str!("advent-of-code-data/2016/day4.txt"), "\0");
    /// 2016 Day 5: How About a Nice Game of Chess?
    pub const DAY5: &str = concat!(include_str!("advent-of-code-data/2016/day5.txt"), "\0");
    /// 2016 Day 6: Signals and Noise
    pub const DAY6: &str = concat!(include_str!("advent-of-code-data/2016/day6.txt"), "\0");
    /// 2016 Day 7: Internet Protocol Version 7
    pub const DAY7: &str = concat!(include_str!("advent-of-code-data/2016/day7.txt"), "\0");
    /// 2016 Day 8: Two-Factor Authentication
    pub const DAY8: &str = concat!(include_str!("advent-of-code-data/2016/day8.txt"), "\0");
    /// 2016 Day 9: Explosives in Cyberspace
    pub const DAY9: &str = concat!(include_str!("advent-of-code-data/2016/day9.txt"), "\0");
    /// 2016 Day 10: Balance Bots
    pub const DAY10: &str = concat!(include_str!("advent-of-code-data/2016/day10.txt"), "\0");
    /// 2016 Day 11: Radioisotope Thermoelectric Generators
    pub const DAY11: &str = concat!(include_str!("advent-of-code-data/2016/day11.txt"), "\0");
    /// 2016 Day 12: Leonardo's Monorail
    pub const DAY12: &str = concat!(include_str!("advent-of-code-data/2016/day12.txt"), "\0");
    /// 2016 Day 13: A Maze of Twisty Little Cubicles
    pub const DAY13: &str = concat!(include_str!("advent-of-code-data/2016/day13.txt"), "\0");
    /// 2016 Day 14: One-Time Pad
    pub const DAY14: &str = concat!(include_str!("advent-of-code-data/2016/day14.txt"), "\0");
    /// 2016 Day 15: Timing is Everything
    pub const DAY15: &str = concat!(include_str!("advent-of-code-data/2016/day15.txt"), "\0");
    /// 2016 Day 16: Dragon Checksum
    pub const DAY16: &str = concat!(include_str!("advent-of-code-data/2016/day16.txt"), "\0");
    /// 2016 Day 17: Two Steps Forward
    pub const DAY17: &str = concat!(include_str!("advent-of-code-data/2016/day17.txt"), "\0");
    /// 2016 Day 18: Like a Rogue
    pub const DAY18: &str = concat!(include_str!("advent-of-code-data/2016/day18.txt"), "\0");
    /// 2016 Day 19: An Elephant Named Joseph
    pub const DAY19: &str = concat!(include_str!("advent-of-code-data/2016/day19.txt"), "\0");
    /// 2016 Day 20: Firewall Rules
    pub const DAY20: &str = concat!(include_str!("advent-of-code-data/2016/day20.txt"), "\0");
    /// 2016 Day 21: Scrambled Letters and Hash
    pub const DAY21: &str = concat!(include_str!("advent-of-code-data/2016/day21.txt"), "\0");
    /// 2016 Day 22: Grid Computing
    pub const DAY22: &str = concat!(include_str!("advent-of-code-data/2016/day22.txt"), "\0");
    /// 2016 Day 23: Safe Cracking
    pub const DAY23: &str = concat!(include_str!("advent-of-code-data/2016/day23.txt"), "\0");
    /// 2016 Day 24: Air Duct Spelunking
    pub const DAY24: &str = concat!(include_str!("advent-of-code-data/2016/day24.txt"), "\0");
    /// 2016 Day 25: Clock Signal
    pub const DAY25: &str = concat!(include_str!("advent-of-code-data/2016/day25.txt"), "\0");

    /// Array containing all inputs in chronological order
    pub const INPUTS: &[&str] = &[
        DAY1, DAY2, DAY3, DAY4, DAY5, DAY6, DAY7, DAY8, DAY9, DAY10, DAY11, DAY12, DAY13, DAY14,
        DAY15, DAY16, DAY17, DAY18, DAY19, DAY20, DAY21, DAY22, DAY23, DAY24, DAY25,
    ];
}

/// Input data for Advent of Code 2017
pub mod aoc_2017 {
    /// 2017 Day 1: Inverse Captcha
    pub const DAY1: &str = concat!(include_str!("advent-of-code-data/2017/day1.txt"), "\0");
    /// 2017 Day 2: Corruption Checksum
    pub const DAY2: &str = concat!(include_str!("advent-of-code-data/2017/day2.txt"), "\0");
    /// 2017 Day 3: Spiral Memory
    pub const DAY3: &str = concat!(include_str!("advent-of-code-data/2017/day3.txt"), "\0");
    /// 2017 Day 4: High-Entropy Passphrases
    pub const DAY4: &str = concat!(include_str!("advent-of-code-data/2017/day4.txt"), "\0");
    /// 2017 Day 5: A Maze of Twisty Trampolines, All Alike
    pub const DAY5: &str = concat!(include_str!("advent-of-code-data/2017/day5.txt"), "\0");
    /// 2017 Day 6: Memory Reallocation
    pub const DAY6: &str = concat!(include_str!("advent-of-code-data/2017/day6.txt"), "\0");
    /// 2017 Day 7: Recursive Circus
    pub const DAY7: &str = concat!(include_str!("advent-of-code-data/2017/day7.txt"), "\0");
    /// 2017 Day 8: I Head You Like Registers
    pub const DAY8: &str = concat!(include_str!("advent-of-code-data/2017/day8.txt"), "\0");
    /// 2017 Day 9: Stream Processing
    pub const DAY9: &str = concat!(include_str!("advent-of-code-data/2017/day9.txt"), "\0");
    /// 2017 Day 10: Knot Hash
    pub const DAY10: &str = concat!(include_str!("advent-of-code-data/2017/day10.txt"), "\0");
    /// 2017 Day 11: Hex Ed
    pub const DAY11: &str = concat!(include_str!("advent-of-code-data/2017/day11.txt"), "\0");
    /// 2017 Day 12: Digital Plumber
    pub const DAY12: &str = concat!(include_str!("advent-of-code-data/2017/day12.txt"), "\0");
    /// 2017 Day 13: Packet Scanners
    pub const DAY13: &str = concat!(include_str!("advent-of-code-data/2017/day13.txt"), "\0");
    /// 2017 Day 14: Disk Defragmentation
    pub const DAY14: &str = concat!(include_str!("advent-of-code-data/2017/day14.txt"), "\0");
    /// 2017 Day 15: Dueling Generators
    pub const DAY15: &str = concat!(include_str!("advent-of-code-data/2017/day15.txt"), "\0");
    /// 2017 Day 16: Permutation Promenade
    pub const DAY16: &str = concat!(include_str!("advent-of-code-data/2017/day16.txt"), "\0");
    /// 2017 Day 17: Spinlock
    pub const DAY17: &str = concat!(include_str!("advent-of-code-data/2017/day17.txt"), "\0");
    /// 2017 Day 18: Duet
    pub const DAY18: &str = concat!(include_str!("advent-of-code-data/2017/day18.txt"), "\0");
    /// 2017 Day 19: A Series of Tubes
    pub const DAY19: &str = concat!(include_str!("advent-of-code-data/2017/day19.txt"), "\0");
    /// 2017 Day 20: Particle Swarm
    pub const DAY20: &str = concat!(include_str!("advent-of-code-data/2017/day20.txt"), "\0");
    /// 2017 Day 21: Fractal Art
    pub const DAY21: &str = concat!(include_str!("advent-of-code-data/2017/day21.txt"), "\0");
    /// 2017 Day 22: Sporifica Virus
    pub const DAY22: &str = concat!(include_str!("advent-of-code-data/2017/day22.txt"), "\0");
    /// 2017 Day 23: Coprocessor Conflagration
    pub const DAY23: &str = concat!(include_str!("advent-of-code-data/2017/day23.txt"), "\0");
    /// 2017 Day 24: Electromagnetic Moat
    pub const DAY24: &str = concat!(include_str!("advent-of-code-data/2017/day24.txt"), "\0");
    /// 2017 Day 25: The Halting Problem
    pub const DAY25: &str = concat!(include_str!("advent-of-code-data/2017/day25.txt"), "\0");

    /// Array containing all inputs in chronological order
    pub const INPUTS: &[&str] = &[
        DAY1, DAY2, DAY3, DAY4, DAY5, DAY6, DAY7, DAY8, DAY9, DAY10, DAY11, DAY12, DAY13, DAY14,
        DAY15, DAY16, DAY17, DAY18, DAY19, DAY20, DAY21, DAY22, DAY23, DAY24, DAY25,
    ];
}

/// Input data for Advent of Code 2018
pub mod aoc_2018 {
    /// 2018 Day 1: Chronal Calibration
    pub const DAY1: &str = concat!(include_str!("advent-of-code-data/2018/day1.txt"), "\0");
    /// 2018 Day 2: Inventory Management System
    pub const DAY2: &str = concat!(include_str!("advent-of-code-data/2018/day2.txt"), "\0");
    /// 2018 Day 3: No Matter How You Slice It
    pub const DAY3: &str = concat!(include_str!("advent-of-code-data/2018/day3.txt"), "\0");
    /// 2018 Day 4: Repose Record
    pub const DAY4: &str = concat!(include_str!("advent-of-code-data/2018/day4.txt"), "\0");
    /// 2018 Day 5: Alchemical Reduction
    pub const DAY5: &str = concat!(include_str!("advent-of-code-data/2018/day5.txt"), "\0");
    /// 2018 Day 6: Chronal Coordinates
    pub const DAY6: &str = concat!(include_str!("advent-of-code-data/2018/day6.txt"), "\0");
    /// 2018 Day 7: The Sum of Its Parts
    pub const DAY7: &str = concat!(include_str!("advent-of-code-data/2018/day7.txt"), "\0");
    /// 2018 Day 8: Memory Maneuver
    pub const DAY8: &str = concat!(include_str!("advent-of-code-data/2018/day8.txt"), "\0");
    /// 2018 Day 9: Marble Mania
    pub const DAY9: &str = concat!(include_str!("advent-of-code-data/2018/day9.txt"), "\0");
    /// 2018 Day 10: The Stars Align
    pub const DAY10: &str = concat!(include_str!("advent-of-code-data/2018/day10.txt"), "\0");
    /// 2018 Day 11: Chronal Charge
    pub const DAY11: &str = concat!(include_str!("advent-of-code-data/2018/day11.txt"), "\0");
    /// 2018 Day 12: Subterranean Sustainability
    pub const DAY12: &str = concat!(include_str!("advent-of-code-data/2018/day12.txt"), "\0");
    /// 2018 Day 13: Mine Cart Madness
    pub const DAY13: &str = concat!(include_str!("advent-of-code-data/2018/day13.txt"), "\0");
    /// 2018 Day 14: Chocolate Charts
    pub const DAY14: &str = concat!(include_str!("advent-of-code-data/2018/day14.txt"), "\0");
    /// 2018 Day 15: Beverage Bandits
    pub const DAY15: &str = concat!(include_str!("advent-of-code-data/2018/day15.txt"), "\0");
    /// 2018 Day 16: Chronal Classification
    pub const DAY16: &str = concat!(include_str!("advent-of-code-data/2018/day16.txt"), "\0");
    /// 2018 Day 17: Reservoir Research
    pub const DAY17: &str = concat!(include_str!("advent-of-code-data/2018/day17.txt"), "\0");
    /// 2018 Day 18: Settlers of The North Pole
    pub const DAY18: &str = concat!(include_str!("advent-of-code-data/2018/day18.txt"), "\0");
    /// 2018 Day 19: Go With The Flow
    pub const DAY19: &str = concat!(include_str!("advent-of-code-data/2018/day19.txt"), "\0");
    /// 2018 Day 20: A Regular Map
    pub const DAY20: &str = concat!(include_str!("advent-of-code-data/2018/day20.txt"), "\0");
    /// 2018 Day 21: Chronal Conversion
    pub const DAY21: &str = concat!(include_str!("advent-of-code-data/2018/day21.txt"), "\0");
    /// 2018 Day 22: Mode Maze
    pub const DAY22: &str = concat!(include_str!("advent-of-code-data/2018/day22.txt"), "\0");
    /// 2018 Day 23: Experimental Emergency Teleportation
    pub const DAY23: &str = concat!(include_str!("advent-of-code-data/2018/day23.txt"), "\0");
    /// 2018 Day 24: Immune System Simulator 20XX
    pub const DAY24: &str = concat!(include_str!("advent-of-code-data/2018/day24.txt"), "\0");
    /// 2018 Day 25: Four-Dimensional Adventure
    pub const DAY25: &str = concat!(include_str!("advent-of-code-data/2018/day25.txt"), "\0");

    /// Array containing all inputs in chronological order
    pub const INPUTS: &[&str] = &[
        DAY1, DAY2, DAY3, DAY4, DAY5, DAY6, DAY7, DAY8, DAY9, DAY10, DAY11, DAY12, DAY13, DAY14,
        DAY15, DAY16, DAY17, DAY18, DAY19, DAY20, DAY21, DAY22, DAY23, DAY24, DAY25,
    ];
}

/// Input data for Advent of Code 2019
pub mod aoc_2019 {
    /// 2019 Day 1: The Tyranny of the Rocket Equation
    pub const DAY1: &str = concat!(include_str!("advent-of-code-data/2019/day1.txt"), "\0");
    /// 2019 Day 2: 1202 Program Alarm
    pub const DAY2: &str = concat!(include_str!("advent-of-code-data/2019/day2.txt"), "\0");
    /// 2019 Day 3: Crossed Wires
    pub const DAY3: &str = concat!(include_str!("advent-of-code-data/2019/day3.txt"), "\0");
    /// 2019 Day 4: Secure Container
    pub const DAY4: &str = concat!(include_str!("advent-of-code-data/2019/day4.txt"), "\0");
    /// 2019 Day 5: Sunny with a Chance of Asteroids
    pub const DAY5: &str = concat!(include_str!("advent-of-code-data/2019/day5.txt"), "\0");
    /// 2019 Day 6: Universal Orbit Map
    pub const DAY6: &str = concat!(include_str!("advent-of-code-data/2019/day6.txt"), "\0");
    /// 2019 Day 7: Amplification Circuit
    pub const DAY7: &str = concat!(include_str!("advent-of-code-data/2019/day7.txt"), "\0");
    /// 2019 Day 8: Space Image Format
    pub const DAY8: &str = concat!(include_str!("advent-of-code-data/2019/day8.txt"), "\0");
    /// 2019 Day 9: Sensor Boost
    pub const DAY9: &str = concat!(include_str!("advent-of-code-data/2019/day9.txt"), "\0");
    /// 2019 Day 10: Monitoring Station
    pub const DAY10: &str = concat!(include_str!("advent-of-code-data/2019/day10.txt"), "\0");
    /// 2019 Day 11: Space Police
    pub const DAY11: &str = concat!(include_str!("advent-of-code-data/2019/day11.txt"), "\0");
    /// 2019 Day 12: The N-Body Problem
    pub const DAY12: &str = concat!(include_str!("advent-of-code-data/2019/day12.txt"), "\0");
    /// 2019 Day 13: Care Package
    pub const DAY13: &str = concat!(include_str!("advent-of-code-data/2019/day13.txt"), "\0");
    /// 2019 Day 14: Space Stoichiometry
    pub const DAY14: &str = concat!(include_str!("advent-of-code-data/2019/day14.txt"), "\0");
    /// 2019 Day 15: Oxygen System
    pub const DAY15: &str = concat!(include_str!("advent-of-code-data/2019/day15.txt"), "\0");
    /// 2019 Day 16: Flawed Frequency Transmission
    pub const DAY16: &str = concat!(include_str!("advent-of-code-data/2019/day16.txt"), "\0");
    /// 2019 Day 17: Set and Forget
    pub const DAY17: &str = concat!(include_str!("advent-of-code-data/2019/day17.txt"), "\0");
    /// 2019 Day 18: Many-Worlds Interpretation
    pub const DAY18: &str = concat!(include_str!("advent-of-code-data/2019/day18.txt"), "\0");
    /// 2019 Day 19: Tractor Beam
    pub const DAY19: &str = concat!(include_str!("advent-of-code-data/2019/day19.txt"), "\0");
    /// 2019 Day 20: Donut Maze
    pub const DAY20: &str = concat!(include_str!("advent-of-code-data/2019/day20.txt"), "\0");
    /// 2019 Day 21: Springdroid Adventure
    pub const DAY21: &str = concat!(include_str!("advent-of-code-data/2019/day21.txt"), "\0");
    /// 2019 Day 22: Slam Shuffle
    pub const DAY22: &str = concat!(include_str!("advent-of-code-data/2019/day22.txt"), "\0");
    /// 2019 Day 23: Category Six
    pub const DAY23: &str = concat!(include_str!("advent-of-code-data/2019/day23.txt"), "\0");
    /// 2019 Day 24: Planet of Discord
    pub const DAY24: &str = concat!(include_str!("advent-of-code-data/2019/day24.txt"), "\0");
    /// 2019 Day 25: Cryostatis
    pub const DAY25: &str = concat!(include_str!("advent-of-code-data/2019/day25.txt"), "\0");

    /// Array containing all inputs in chronological order
    pub const INPUTS: &[&str] = &[
        DAY1, DAY2, DAY3, DAY4, DAY5, DAY6, DAY7, DAY8, DAY9, DAY10, DAY11, DAY12, DAY13, DAY14,
        DAY15, DAY16, DAY17, DAY18, DAY19, DAY20, DAY21, DAY22, DAY23, DAY24, DAY25,
    ];
}

/// Input data for Advent of Code 2020
pub mod aoc_2020 {
    /// 2020 Day 1: Report Repair
    pub const DAY1: &str = concat!(include_str!("advent-of-code-data/2020/day1.txt"), "\0");
    /// 2020 Day 2: Password Philosophy
    pub const DAY2: &str = concat!(include_str!("advent-of-code-data/2020/day2.txt"), "\0");
    /// 2020 Day 3: Toboggan Trajectory
    pub const DAY3: &str = concat!(include_str!("advent-of-code-data/2020/day3.txt"), "\0");
    /// 2020 Day 4: Passport Processing
    pub const DAY4: &str = concat!(include_str!("advent-of-code-data/2020/day4.txt"), "\0");
    /// 2020 Day 5: Binary Boarding
    pub const DAY5: &str = concat!(include_str!("advent-of-code-data/2020/day5.txt"), "\0");
    /// 2020 Day 6: Custom Customs
    pub const DAY6: &str = concat!(include_str!("advent-of-code-data/2020/day6.txt"), "\0");
    /// 2020 Day 7: Handy Haversacks
    pub const DAY7: &str = concat!(include_str!("advent-of-code-data/2020/day7.txt"), "\0");
    /// 2020 Day 8: Handheld Halting
    pub const DAY8: &str = concat!(include_str!("advent-of-code-data/2020/day8.txt"), "\0");
    /// 2020 Day 9: Encoding Error
    pub const DAY9: &str = concat!(include_str!("advent-of-code-data/2020/day9.txt"), "\0");
    /// 2020 Day 10: Adapter Array
    pub const DAY10: &str = concat!(include_str!("advent-of-code-data/2020/day10.txt"), "\0");
    /// 2020 Day 11: Seating System
    pub const DAY11: &str = concat!(include_str!("advent-of-code-data/2020/day11.txt"), "\0");
    /// 2020 Day 12: Rain Risk
    pub const DAY12: &str = concat!(include_str!("advent-of-code-data/2020/day12.txt"), "\0");
    /// 2020 Day 13: Shuttle Search
    pub const DAY13: &str = concat!(include_str!("advent-of-code-data/2020/day13.txt"), "\0");
    /// 2020 Day 14: Docking Data
    pub const DAY14: &str = concat!(include_str!("advent-of-code-data/2020/day14.txt"), "\0");
    /// 2020 Day 15: Rambunctious Recitation
    pub const DAY15: &str = concat!(include_str!("advent-of-code-data/2020/day15.txt"), "\0");
    /// 2020 Day 16: Ticket Translation
    pub const DAY16: &str = concat!(include_str!("advent-of-code-data/2020/day16.txt"), "\0");
    /// 2020 Day 17: Conway Cubes
    pub const DAY17: &str = concat!(include_str!("advent-of-code-data/2020/day17.txt"), "\0");
    /// 2020 Day 18: Operation Order
    pub const DAY18: &str = concat!(include_str!("advent-of-code-data/2020/day18.txt"), "\0");
    /// 2020 Day 19: Monster Messages
    pub const DAY19: &str = concat!(include_str!("advent-of-code-data/2020/day19.txt"), "\0");
    /// 2020 Day 20: Jurassic Jigsaw
    pub const DAY20: &str = concat!(include_str!("advent-of-code-data/2020/day20.txt"), "\0");
    /// 2020 Day 21: Allergen Assessment
    pub const DAY21: &str = concat!(include_str!("advent-of-code-data/2020/day21.txt"), "\0");
    /// 2020 Day 22: Crab Combat
    pub const DAY22: &str = concat!(include_str!("advent-of-code-data/2020/day22.txt"), "\0");
    /// 2020 Day 23: Crab Cups
    pub const DAY23: &str = concat!(include_str!("advent-of-code-data/2020/day23.txt"), "\0");
    /// 2020 Day 24: Lobby Layout
    pub const DAY24: &str = concat!(include_str!("advent-of-code-data/2020/day24.txt"), "\0");
    /// 2020 Day 25: Combo Breaker
    pub const DAY25: &str = concat!(include_str!("advent-of-code-data/2020/day25.txt"), "\0");

    /// Array containing all inputs in chronological order
    pub const INPUTS: &[&str] = &[
        DAY1, DAY2, DAY3, DAY4, DAY5, DAY6, DAY7, DAY8, DAY9, DAY10, DAY11, DAY12, DAY13, DAY14,
        DAY15, DAY16, DAY17, DAY18, DAY19, DAY20, DAY21, DAY22, DAY23, DAY24, DAY25,
    ];
}

/// Input data for Advent of Code 2020
pub mod aoc_2021 {
    /// 2021 Day 1: Sonar Sweep
    pub const DAY1: &str = concat!(include_str!("advent-of-code-data/2021/day1.txt"), "\0");

    /// 2021 Day 2: Dive!
    pub const DAY2: &str = concat!(include_str!("advent-of-code-data/2021/day2.txt"), "\0");

    /// 2021 Day 3: Binary Diagnostic
    pub const DAY3: &str = concat!(include_str!("advent-of-code-data/2021/day3.txt"), "\0");

    /// 2021 Day 4: Giant Squid
    pub const DAY4: &str = concat!(include_str!("advent-of-code-data/2021/day4.txt"), "\0");

    /// 2021 Day 5: Hydrothermal Venture
    pub const DAY5: &str = concat!(include_str!("advent-of-code-data/2021/day5.txt"), "\0");

    /// 2021 Day 6: Lanternfish
    pub const DAY6: &str = concat!(include_str!("advent-of-code-data/2021/day6.txt"), "\0");

    /// 2021 Day 7: The Treachery of Whales
    pub const DAY7: &str = concat!(include_str!("advent-of-code-data/2021/day7.txt"), "\0");

    /// 2021 Day 8: Seven Segment Search
    pub const DAY8: &str = concat!(include_str!("advent-of-code-data/2021/day8.txt"), "\0");

    /// 2021 Day 9: Smoke Basin
    pub const DAY9: &str = concat!(include_str!("advent-of-code-data/2021/day9.txt"), "\0");

    /// 2021 Day 10: Syntax Scoring
    pub const DAY10: &str = concat!(include_str!("advent-of-code-data/2021/day10.txt"), "\0");

    /// 2021 Day 11: Dumbo Octopus
    pub const DAY11: &str = concat!(include_str!("advent-of-code-data/2021/day11.txt"), "\0");

    /// 2021 Day 12: Passage Pathing
    pub const DAY12: &str = concat!(include_str!("advent-of-code-data/2021/day12.txt"), "\0");

    /// 2021 Day 13: Transparent Origami
    pub const DAY13: &str = concat!(include_str!("advent-of-code-data/2021/day13.txt"), "\0");

    /// 2021 Day 14: Extended Polymerization
    pub const DAY14: &str = concat!(include_str!("advent-of-code-data/2021/day14.txt"), "\0");

    /// 2021 Day 15: Chiton
    pub const DAY15: &str = concat!(include_str!("advent-of-code-data/2021/day15.txt"), "\0");

    /// 2021 Day 16: Packet Decoder
    pub const DAY16: &str = concat!(include_str!("advent-of-code-data/2021/day16.txt"), "\0");

    /// 2021 Day 17: Trick Shot
    pub const DAY17: &str = concat!(include_str!("advent-of-code-data/2021/day17.txt"), "\0");

    /// Array containing all inputs in chronological order
    pub const INPUTS: &[&str] = &[
        DAY1, DAY2, DAY3, DAY4, DAY5, DAY6, DAY7, DAY8, DAY9, DAY10, DAY11, DAY12, DAY13, DAY14,
        DAY15, DAY16, DAY17,
    ];
}

#[repr(C)]
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq)]
pub enum Year {
    TwentyFifteen = 2015,
    TwentySixteen,
    TwentySeventeen,
    TwentyEighteen,
    TwentyNineteen,
    TwentyTwenty,
    TwentyTwentyOne,
}

#[repr(C)]
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq)]
pub enum Day {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

/// Array containing refrences to all inputs slices in chronological order
const INPUTS: &[&[&str]] = &[
    aoc_2015::INPUTS,
    aoc_2016::INPUTS,
    aoc_2017::INPUTS,
    aoc_2018::INPUTS,
    aoc_2019::INPUTS,
    aoc_2020::INPUTS,
    aoc_2021::INPUTS,
];

/// Get the input for the requested day and year
pub const fn get_input(year: Year, day: Day) -> Option<&'static str> {
    let y = year as usize - 2015;
    let d = day as usize - 1;

    if y > INPUTS.len() {
        return None;
    }

    let inputs = INPUTS[y];

    if d > inputs.len() {
        return None;
    }

    Some(inputs[d])
}
