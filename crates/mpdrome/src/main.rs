use mpdrome_config::Settings;
use mpdrome_mpd::Mpd;
use mpdrome_subsonic::SubSonicClient;

fn main() -> color_eyre::Result<()> {
    let config = Settings::get()?;
    let navidrome_client = SubSonicClient::new(
        &config.navidrome.user,
        &config.navidrome.password,
        &config.navidrome.url,
    );

    Mpd::new("127.0.0.1:6600", navidrome_client)
        .start()
        .unwrap();
    Ok(())
}
