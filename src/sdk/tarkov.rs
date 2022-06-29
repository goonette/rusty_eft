use ::{c_str, cast};
use sdk::MonoLib;

pub struct World {
    base_addr: *const usize,
    class: *const usize,
    mono: MonoLib,
}

impl World {
    pub fn new(base_addr: *const usize, mono: MonoLib) -> World {
        let world = unsafe { *cast!(base_addr as usize + 0x10, usize) as *const usize };
        let temp_0 = unsafe { *cast!(world as usize + 0x30, usize) as *const usize };
        let temp_1 = unsafe { *cast!(temp_0 as usize + 0x18, usize) as *const usize };

        let local_game_world = unsafe { *cast!(temp_1 as usize + 0x28, usize) as *const usize };
        println!("Local Game World = {:?}", local_game_world);

        let game_world_class = mono.get_class(c_str!("Assembly-CSharp"), c_str!("EFT"), c_str!("GameWorld"));
        println!("Game World Class = {:?}", game_world_class);

        World { base_addr: local_game_world, class: game_world_class, mono }
    }

    pub fn get_players(&self) -> Vec<*const usize> {
        let registered_players_addr = self.mono.get_field(self.class, "RegisteredPlayers");
        let registered_players = unsafe { *cast!(self.base_addr as usize + self.mono.field_get_offset(registered_players_addr) as usize, usize) as *const usize };

        let player_base = unsafe { *cast!(registered_players as usize + 0x10, usize ) as *const usize };
        let player_count = unsafe { *cast!(registered_players as usize + 0x18, i32) };

        let mut i = 0;
        let mut vec: Vec<*const usize> = vec![];

        while i < player_count {
            let current_entry = unsafe { *cast!(player_base as usize + (0x20 + (i * 0x8 )) as usize, usize) as *const usize };
            vec.push(current_entry);
            i += 1;
        }

        vec
    }
}