pub fn verse(n: i32) -> String {
    match n {
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1).to_string(),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = "".to_string();

    for n in (end+1..start+1).rev() {
        song.push_str(&verse(n));
        song.push_str("\n");
    }
    song.push_str(&verse(end));

    song.to_string()
}
