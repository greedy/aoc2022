use std::collections::HashMap;

#[derive(Debug)]
pub struct Room {
    name: String,
    tunnels: Vec<String>,
    valve_rate: u32,
}

impl Room {
    pub fn parse(s: &str) -> nom::IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::sequence::{preceded, tuple};
        use nom::multi::separated_list1;
        use nom::character::complete::{u32, alpha1};
        use nom::branch::alt;
        use nom::Parser;

        tuple((
                preceded(tag("Valve "), alpha1.map(|s: &str| s.to_owned())),
                preceded(tag(" has flow rate="), u32),
                preceded(alt((tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "))), separated_list1(tag(", "), alpha1.map(|s: &str| s.to_owned())))
        )).map(|(name, valve_rate, tunnels)| Room { name, valve_rate, tunnels }).parse(s)
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn valve_rate(&self) -> u32 {
        self.valve_rate
    }

    pub fn tunnels(&self) -> impl Iterator<Item = &str> {
        self.tunnels.iter().map(String::as_str)
    }
}

pub struct Caverns {
    rooms: Vec<Room>,
    name_index: HashMap<String, usize>
}

impl Caverns {
    pub fn new<I:IntoIterator<Item = Room>>(rooms: I) -> Self {
        let rooms: Vec<Room> = rooms.into_iter().collect();
        let name_index = rooms.iter().enumerate().map(|(ix, room)| (room.name.clone(), ix)).collect();
        Caverns { rooms, name_index }
    }

    pub fn rooms<'a>(&'a self) -> impl Iterator<Item = &'a Room> {
        self.rooms.iter()
    }

    pub fn neighbors<'a>(&'a self, room: &'a Room) -> impl Iterator<Item = Result<&'a Room, &'a str>> + 'a {
        self.neighbor_ids(room).map(|neighbor| neighbor.map(|i| &self.rooms[i]))
    }

    pub fn neighbor_ids<'a>(&'a self, room: &'a Room) -> impl Iterator<Item = Result<usize, &'a str>> + 'a {
        room.tunnels.iter().map(|dest| self.name_index.get(dest).map(|i| i.to_owned()).ok_or(dest.as_str()))
    }

    pub fn room_id(&self, name: &str) -> Option<usize> {
        self.name_index.get(name).copied()
    }

    pub fn room_named(&self, name: &str) -> Option<&Room> {
        self.room_id(name).map(|id| self.room(id))
    }

    pub fn room(&self, id: usize) -> &Room {
        &self.rooms[id]
    }
}

