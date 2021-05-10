use bindings::{
    Windows::Foundation::Uri,
    Windows::Web::Syndication::SyndicationClient,
    Windows::Win32::WindowsAndMessaging::*,
};

fn main() -> windows::Result<()> {
    let uri = Uri::CreateUri("https://blogs.windows.com/feed")?;
    let client = SyndicationClient::new()?;
    let feed = client.RetrieveFeedAsync(uri)?.get()?;

    let mut result = String::new();
    for item in feed.Items()? {
        println!("{}", item.Title()?.Text()?);
        result = format!("{}{}\n", result, item.Title()?.Text()?);
    }

    unsafe {
        MessageBoxA(None, result, "List of Feed", MESSAGEBOX_STYLE::MB_OK);
    }

    Ok(())
}
