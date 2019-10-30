/*
Simple, random dungeon generator written in Rust. Originally described by Mike Anderson at
http://www.roguebasin.com/index.php?title=Dungeon-Building_Algorithm

Sample usage:

let mut d = Dungeon::new(50, 50);
let max_features = 35;
d.generate(max_features);

To see the output, call d._print_dungeon()
*/

use std::slice::Iter;
use rng;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Tile {
    Unused,
    Floor,
    Corridor,
    Wall,
    ClosedDoor,
    OpenDoor,
    Exit,
    Entrance,
}

#[derive(Debug, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    // iterator over direction variants
    pub fn iterator() -> Iter<'static, Dir> {
        static DIR: [Dir;  4] = [Dir::North, Dir::South, Dir::East, Dir::West];
        DIR.iter()
    }

    pub fn get_random_dir<'a>()-> &'a Dir {
        Dir::iterator().nth(rng::inclusive_random(0, 3) as usize).unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
}

impl Rect {
    fn new(x: isize, y: isize, width: isize, height: isize) -> Rect {
        Rect { x: x, y: y, width: width, height: height }
    }
}

pub struct Dungeon {
    width: isize,
    height: isize,
    tiles: Vec<Tile>,
    rooms: Vec<Rect>,
    exits: Vec<Rect>,
}

impl Dungeon {
    pub fn new(width: isize, height: isize) -> Dungeon {
        let mut tiles = Vec::new();
        for _x in 1..width*height+1 {
            tiles.push(Tile::Unused);
        }

        Dungeon { width: width, height: height, tiles: tiles, rooms: Vec::new(),
                    exits: Vec::new() }
    }

    fn _print_dungeon(&self) {
        for y in 1..self.height {
            for x in 1..self.width {
                print!("{}", self._get_tile_icon(self.get_tile(x, y)));
            }
            println!("");
        }
    }

    fn _get_tile_icon(&self, tile: Tile) -> char {
        match tile {
            Tile::Floor =>      '.',
            Tile::Corridor =>   ',',
            Tile::Wall =>       '#',
            Tile::ClosedDoor => '+',
            Tile::OpenDoor =>   '-',
            Tile::Exit =>       '>',
            Tile::Entrance =>   '<',
            Tile::Unused =>     ' ', 
        }
    }

    pub fn generate(&mut self, maxfeatures: isize) {
        let x = self.width;
        let y = self.height;
        if !self.make_room(x / 2, y / 2, Dir::get_random_dir(), true) {
            println!("unable to place first room!");
        }

        for x in 1..maxfeatures {
            if !self.has_exits() {
                println!("unable to place more features, placed {}.", x);
                break;
            }
        }

        if !self.place_object(Tile::Exit) {
            println!("unable to place exit");
        }

        if !self.place_object(Tile::Entrance) {
            println!("unable to place entrance");
        }
    }

    fn get_tile(&self, x: isize, y: isize) -> Tile {
        if (x < 0) || (y < 0) || (x >= self.width as isize) || (y >= self.height as isize) {
            return Tile::Unused
        }

        self.tiles[x as usize + y as usize * self.width as usize]
    }

    fn set_tile(&mut self, x: isize, y: isize, tile: Tile) {
        self.tiles[x as usize + y as usize * self.width as usize] = tile;
    }

    fn has_exits(&mut self) -> bool {
        for _i in 0..1000 {
            if self.exits.is_empty() {
                break;
            }

            // pick a random side of a room/corridor
            let r: isize = rng::exclusive_random(self.exits.len() as isize);
            let x: isize = rng::inclusive_random(self.exits[r as usize].x, self.exits[r as usize].x + 
                self.exits[r as usize].width - 1);
            let y: isize = rng::inclusive_random(self.exits[r as usize].y, self.exits[r as usize].y + 
                self.exits[r as usize].height - 1);

            for dir in Dir::iterator() {
                if self.create_feature(x, y, dir) {
                    self.exits.remove(r as usize);
                    return true
                }
            }
        }
        false
    }

    fn create_feature(&mut self, x: isize, y: isize, dir: &Dir) -> bool {
        let room_chance: isize = 50;
        let mut dx: isize = 0;
        let mut dy: isize = 0;

        if *dir == Dir::North {
            dy = 1;
        }
        
        else if *dir == Dir::South {
            dy = -1;
        }

        else if *dir == Dir::East {
            dx = -1;
        }

        else if *dir == Dir::West {
            dx = 1;
        }

        if self.get_tile(x as isize + dx, y as isize + dy) != Tile::Floor && self.get_tile(x as isize + dx, y as isize + dy) != Tile::Corridor {
            return false
        }

        if rng::exclusive_random(100) < room_chance {
            if self.make_room(x, y, &dir, false) {
                self.set_tile(x, y, Tile::ClosedDoor);

                return true
            }
        }

        else {
            if self.make_corridor(x, y, dir) {
                if self.get_tile(x as isize + dx, y as isize + dy) == Tile::Floor {
                    self.set_tile(x, y, Tile::ClosedDoor);
                }

                else {
                    self.set_tile(x, y, Tile::Corridor);

                    return true
                }
            }
        }
        return false
    }

    fn make_room(&mut self, x: isize, y: isize, dir: &Dir, firstroom: bool) -> bool {
        let minsize: isize = 3;
        let maxsize: isize = 16;

        let mut room: Rect = Rect::new(0, 0, rng::inclusive_random(minsize, maxsize), rng::inclusive_random(minsize, maxsize));

        if *dir == Dir::North {
            room.x = x - room.width / 2;
            room.y = y - room.height;
        }

        else if *dir == Dir::South {
            room.x = x - room.width /2;
            room.y = y + 1;
        }

        else if *dir == Dir::East {
            room.x = x + 1;
            room.y = y - room.height / 2;
        }

        else if *dir == Dir::West {
            room.x = x - room.width;
            room.y = y - room.height / 2;
        }

        if self.place_rect(&room, Tile::Floor) {
            self.rooms.push(room);

            if *dir != Dir::South || firstroom {
                self.exits.push(Rect::new(room.x, room.y - 1, room.width, 1));
            }

            if *dir != Dir::North || firstroom {
                self.exits.push(Rect::new(room.x, room.y + room.height, room.width, 1));
            }

            if *dir != Dir::East || firstroom {
                self.exits.push(Rect::new(room.x - 1, room.y, 1, room.height));
            }

            if *dir != Dir::West || firstroom {
                self.exits.push(Rect::new(room.x + room.width, room.y, 1, room.height));
            }

            return true
        }

        return false
    }

    fn make_corridor(&mut self, x: isize, y: isize, dir: &Dir) -> bool {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        let minlength = 3;
        let maxlength = 10;

        let mut corridor = Rect::new(x, y, 0, 0);

        if rng.gen() { // horizontal
            corridor.width = rng::inclusive_random(minlength, maxlength);
            corridor.height = 1;

            if *dir == Dir::North {
                corridor.y = y - 1;
                if rng.gen() { // west
                    corridor.x = x - corridor.width + 1;
                }
            }

            else if *dir == Dir::South {
                corridor.y = y + 1;

                if rng.gen() { // west
                    corridor.x = x - corridor.width + 1;
                }
            }

            else if *dir == Dir::East {
                corridor.x = x + 1;
            }

            else if *dir == Dir::West {
                corridor.x = x - corridor.width + 1;
            }
        }

        else { // vertical
            corridor.width = 1;
            corridor.height = rng::inclusive_random(minlength, maxlength);

            if *dir == Dir::North {
                corridor.y = y - corridor.height;
            }

            else if *dir == Dir::South {
                corridor.y = y + 1;
            }

            else if *dir == Dir::East {
                corridor.x = x + 1;

                if rng.gen() { // north
                    corridor.y = y - corridor.height + 1;
                }
            }

            else if *dir == Dir::West {
                corridor.x = x - 1;

                if rng.gen() { // north
                    corridor.y = y - corridor.height + 1;
                }
            }
        }

        if self.place_rect(&corridor, Tile::Corridor) {
            if *dir != Dir::South && corridor.width != 1 { // north side
                self.exits.push(Rect::new(corridor.x, corridor.y - 1, corridor.width, 1));
            }

            if *dir != Dir::North && corridor.width != 1 { // south side
                self.exits.push(Rect::new(corridor.x, corridor.y + corridor.height, corridor.width, 1));
            }

            if *dir != Dir::East && corridor.height != 1 { // west side
                self.exits.push(Rect::new(corridor.x - 1, corridor.y, 1, corridor.height));
            }

            if *dir != Dir::West && corridor.height != 1 { // east side
                self.exits.push(Rect::new(corridor.x + corridor.width, corridor.y, 1, corridor.height));
            }

            return true
        }

        return false
    }

    fn place_rect(&mut self, rect: &Rect, tile: Tile) -> bool {
        // ensure rect is placed within the boundaries of the dungeon
        if (rect.x <= 1) || (rect.y <= 1) || (rect.x + rect.width > self.width - 1) || (rect.y + rect.height > self.height - 1) {
            return false
        }

        for y in rect.y..rect.y+rect.height {
            for x in rect.x..rect.x + rect.width {
                if self.get_tile(x, y) != Tile::Unused {
                    return false // this area is already in use
                }
            }
        }

        // checks have passed, we can place a rect here
        for y in rect.y-1..rect.y+rect.height+1 {
            for x in rect.x-1..rect.x+rect.width+1 {
                // fill boundaries of rect with walls
                if (x == rect.x - 1) || (y == rect.y - 1) || (x == rect.x + rect.width) || (y == rect.y + rect.height) {
                    self.set_tile(x, y, Tile::Wall);
                }

                // fill rect with appropriate tiles
                else {
                    self.set_tile(x, y, tile);
                }
            }
        }

        return true
    }

    fn place_object(&mut self, tile: Tile) -> bool {
        if self.rooms.is_empty() {
            return false
        }

        let r: isize = rng::exclusive_random(self.rooms.len() as isize);
        let x: isize = rng::inclusive_random(self.rooms[r as usize].x + 1, self.rooms[r as usize].x + self.rooms[r as usize].width - 2);
        let y: isize = rng::inclusive_random(self.rooms[r as usize].y + 1, self.rooms[r as usize].y + self.rooms[r as usize].height - 2);

        if self.get_tile(x, y) == Tile::Floor {
            self.set_tile(x, y, tile);

            self.rooms.remove(r as usize);

            return true
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use dungeon::*;
    
    #[test]
    fn test_dungeon() {
        let mut d: Dungeon = Dungeon::new(100, 100);
        let max_features: isize = 78;

        d.generate(max_features);
        // must use cargo test -- --nocapture to see this output
        d._print_dungeon();
    }
}