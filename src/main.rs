fn main() -> std::io::Result<()> {
    use std::fs;
    
    let music_dir = fs:::wq

    let metadata = fs::metadata("/home/ubuntu/music/Music/Laffey - Summer Nights.mp3")?;

    println!("{:?}", metadata.file_type());
    Ok(())
}

