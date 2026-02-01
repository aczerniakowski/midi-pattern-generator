#[derive(Debug, Clone, Copy)]
pub enum ScaleType {
    Major,
    Minor,
}

pub fn notes_in_scale(root: i32, scale: ScaleType) -> Vec<i32> {
    let intervals = match scale {
        ScaleType::Major => vec![0,2,4,5,7,9,11],
        ScaleType::Minor => vec![0,2,3,5,7,8,10],
    };
    intervals.iter().map(|i| root + i).collect()
}