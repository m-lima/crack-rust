use crate::options;

static MD5: &str = include!(concat!(env!("OUT_DIR"), "/cl/md5.rs"));
static SHA256: &str = include!(concat!(env!("OUT_DIR"), "/cl/sha256.rs"));

pub fn get_source_for<'a>(algorithm: &options::Algorithm) -> &'a str {
    match algorithm {
        options::Algorithm::MD5 => super::sources::MD5,
        options::Algorithm::SHA256 => super::sources::SHA256,
    }
}
