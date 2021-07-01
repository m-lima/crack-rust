// TODO: Remove this once Path is used instead of Ident in qmetaobject_impl
use qmetaobject::QObject;

use crate::secrets;

#[derive(QObject, Default)]
pub struct Cracker {
    base: qmetaobject::qt_base_class!(trait QObject),
    crack: qmetaobject::qt_method!(
        #[allow(
            clippy::unused_self,
            clippy::needless_pass_by_value,
            clippy::too_many_arguments
        )]
        fn crack(
            &self,
            prefix: String,
            length: u8,
            custom_salt: bool,
            salt: String,
            useSha256: bool,
            auto_device: bool,
            useGpu: bool,
            hashes: qmetaobject::QVariantList,
            files: qmetaobject::QVariantList,
        ) {
            println!(
                "Prefix: {}, Length: {}, Salt: {}, Algorithm: {}, Device: {}, Hashes: {}, Files: {}",
                prefix,
                length,
                if custom_salt {
                    salt.as_str()
                } else {
                    secrets::SALT
                },
                if useSha256 { "SHA256" } else { "MD5" },
                if auto_device {
                    "AUTO"
                } else if useGpu {
                    "GPU"
                } else {
                    "CPU"
                },
                hashes.len(),
                files.len(),
            );
        }
    ),
}

impl qmetaobject::QSingletonInit for Cracker {
    fn init(&mut self) {}
}
