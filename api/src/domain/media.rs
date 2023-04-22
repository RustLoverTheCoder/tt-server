pub enum ApiMediaFormat {
    BlobUrl,
    Progressive,
    Stream,
    DownloadUrl,
    Text,
}

pub type ApiParsedMedia = String;
pub type ApiPreparedMedia = String;