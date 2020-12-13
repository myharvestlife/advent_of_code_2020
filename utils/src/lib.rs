pub mod input_converter {
    pub fn to_vec64(input: String) -> Vec<u64> {
        return input.split("\n").filter_map(|n| n.parse().ok()).collect();
    }

    pub fn to_vec32(input: String) -> Vec<u32> {
        return input.split("\n").filter_map(|n| n.parse().ok()).collect();
    }

    pub fn to_char_2d_vector(input: String) -> Vec<Vec<char>> {
        let rows: Vec<&str> = input.split("\n").filter(|n| n.len() > 0).collect();

        return rows.iter().map(|r| r.chars().collect()).collect();
    }

    pub fn to_string_vec(input: String) -> Vec<String> {
        return input.split("\n").filter(|n| n.len() > 0).map(|s| s.to_string()).collect();
    }
}
