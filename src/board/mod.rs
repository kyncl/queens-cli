pub mod empty;
pub mod player_pos;
pub mod queen;
pub mod swapper;

#[derive(Debug)]
pub struct Board {
    pub size: (u8, u8),
    pub regions: Vec<Vec<(u8, u8)>>,
    pub queen_pos: Vec<(u8, u8)>,
    pub empty_pos: Vec<(u8, u8)>,
    pub queen_skin: String,
    pub empty_skin: String,
}

impl Board {
    pub fn load_board() -> Board {
        let loaded_board = "6x6|\
        0:0,0;1,0;2,0;2,1;\
        1:0,1;0,2;0,3;1,3;\
        2:3,0;1,1;2,2;3,1;1,2;3,2;\
        3:4,0;5,0;4,1;4,2;2,3;3,3;4,3;\
        4:5,1;5,2;5,3;5,4;\
        5:0,4;1,4;2,4;3,4;4,4;0,5;1,5;2,5;3,5;4,5;5,5;\
        | :|X:";
        let sections: Vec<&str> = loaded_board.split('|').collect();
        let size_parts: Vec<u8> = sections[0]
            .split('x')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let size = (size_parts[0], size_parts[1]);

        let mut regions = Vec::new();
        if let Some(reg_sec) = sections.get(1) {
            let parts: Vec<&str> = reg_sec.split(':').collect();
            for i in 1..parts.len() {
                let mut chunk = parts[i];
                if i < parts.len() - 1 {
                    chunk = &chunk[..chunk.len() - 1];
                }

                let coords = parse_coords(chunk);
                if !coords.is_empty() {
                    regions.push(coords);
                }
            }
        }

        let mut queen_pos = Vec::new();
        let mut queen_skin = String::from("?");
        if let Some(q_sec) = sections.get(2)
            && let Some((skin, coords_raw)) = q_sec.split_once(':')
        {
            queen_skin = skin.trim().to_string();
            queen_pos = parse_coords(coords_raw);
        }

        let mut empty_pos = Vec::new();
        let mut empty_skin = String::from("X");
        if let Some(e_sec) = sections.get(3)
            && let Some((skin, coords_raw)) = e_sec.split_once(':')
        {
            empty_skin = skin.trim().to_string();
            empty_pos = parse_coords(coords_raw);
        }

        Board {
            size,
            regions,
            queen_pos,
            empty_pos,
            queen_skin,
            empty_skin,
        }
    }
}

fn parse_coords(input: &str) -> Vec<(u8, u8)> {
    input
        .split(';')
        .filter_map(|p| {
            let n: Vec<u8> = p.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            if n.len() == 2 {
                Some((n[0], n[1]))
            } else {
                None
            }
        })
        .collect()
}
