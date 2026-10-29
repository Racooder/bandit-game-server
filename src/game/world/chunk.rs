pub struct Chunk {
    // height data (8x8 subgrid?)
    // some kind of playerdata (isplayerchunk / player id / player pos)
    // biome data (probably one biome per chunk)
    // poi (Maybe split off npc cities)
    // entities (caravans / bandit groups / other hazards like animals etc.) (maybe split into caravans and entities)
    // weather (later) (probably displayed independently for less rigidness but affects actions in chunk)
    // events (active attacks on caravans or natural events like a landslide) (maybe name different)
}

impl Chunk {
    pub fn get_visible_data(/* Some kind of player id */) {
        // Get chunk data visible to the player (For example cull entities if the player has no presence in the chunk)
    }
}
