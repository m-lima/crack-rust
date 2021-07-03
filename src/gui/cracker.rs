use qmetaobject::QObject;

use crate::channel;
use crate::decrypt;
use crate::hash;
use crate::options;
use crate::secrets;

#[derive(QObject, Default)]
pub struct Cracker {
    base: qmetaobject::qt_base_class!(trait QObject),
    running: qmetaobject::qt_property!(bool),
    progressed: qmetaobject::qt_signal!(progress: u8),
    found: qmetaobject::qt_signal!(input: String, output: String),
    error: qmetaobject::qt_signal!(message: String),
    crack: qmetaobject::qt_method!(
        fn(
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
        )
    ),
}

#[allow(
    clippy::needless_pass_by_value,
    clippy::too_many_arguments,
    clippy::fn_params_excessive_bools
)]
impl Cracker {
    fn crack(
        &self,
        prefix: String,
        length: u8,
        custom_salt: bool,
        salt: String,
        use_sha256: bool,
        auto_device: bool,
        use_gpu: bool,
        input: qmetaobject::QVariantList,
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
            if use_sha256 { "SHA256" } else { "MD5" },
            if auto_device {
                "AUTO"
            } else if use_gpu {
                "GPU"
            } else {
                "CPU"
            },
            input.len(),
            files.len(),
        );

        if use_sha256 {
            self.crack_inner::<hash::sha256::Hash>(
                prefix,
                length,
                custom_salt,
                salt,
                auto_device,
                use_gpu,
                input,
                files,
            );
        } else {
            self.crack_inner::<hash::md5::Hash>(
                prefix,
                length,
                custom_salt,
                salt,
                auto_device,
                use_gpu,
                input,
                files,
            );
        }
    }

    fn crack_inner<H: hash::Hash>(
        &self,
        prefix: String,
        length: u8,
        custom_salt: bool,
        salt: String,
        auto_device: bool,
        use_gpu: bool,
        input: qmetaobject::QVariantList,
        files: qmetaobject::QVariantList,
    ) {
        use qmetaobject::QMetaType;

        println!("Craking in rust");
        // TODO: Lots of copying happening here
        let input = input
            .into_iter()
            .filter_map(|v| qmetaobject::QString::from_qvariant(v.clone()))
            .filter_map(|s| H::from_str(&s.to_string()).ok())
            .collect();

        let files = files
            .into_iter()
            .filter_map(|v| qmetaobject::QString::from_qvariant(v.clone()))
            .map(|s| std::path::PathBuf::from(&s.to_string()))
            .collect();

        let maybe_salt = if custom_salt { Some(salt) } else { None };
        let maybe_device = if auto_device {
            None
        } else if use_gpu {
            Some(options::Device::GPU)
        } else {
            Some(options::Device::CPU)
        };
        println!("Parameters built");

        // TODO: Need to release the calling thread. Must not block
        if let Err(err) =
            options::Decrypt::new(input, files, maybe_salt, length, prefix, None, maybe_device)
                .and_then(|ref options| decrypt::execute(options, self))
        {
            println!("Failure: {}", err);
            self.error(err.to_string())
        }
    }
}

impl qmetaobject::QSingletonInit for Cracker {
    fn init(&mut self) {
        self.running = true;
    }
}

unsafe impl Sync for Cracker {}

impl channel::Channel for Cracker {
    fn progress(&self, progress: u8) {
        self.progressed(progress)
    }

    fn result(&self, input: &str, output: &str) {
        self.found(input.into(), output.into())
    }

    fn should_terminate(&self) -> bool {
        self.running
    }
}
