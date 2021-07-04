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
            self.crack_algorithm::<hash::sha256::Hash>(
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
            self.crack_algorithm::<hash::md5::Hash>(
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

    fn crack_algorithm<H: hash::Hash>(
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

        let options = match options::Decrypt::new(
            input,
            files,
            maybe_salt,
            length,
            prefix,
            None,
            maybe_device,
        ) {
            Ok(options) => options,
            Err(err) => {
                self.error(err.to_string());
                return;
            }
        };

        let channel = Channel::new(self);

        std::thread::spawn(move || {
            // TODO: Need to communicate failure
            // TODO: Need to communicate Summary?
            let _ignored = decrypt::execute(&options, &channel);
        });
    }
}

struct Channel {
    progress_callback: Box<dyn Fn(u8) + Send + Sync>,
    result_callback: Box<dyn Fn((String, String)) + Send + Sync>,
}

impl Channel {
    fn new(cracker: &Cracker) -> Self {
        let ptr = qmetaobject::QPointer::from(&*cracker);
        let progress = qmetaobject::queued_callback(move |progress| {
            if let Some(this) = ptr.as_pinned() {
                this.borrow().progressed(progress);
            }
        });
        let ptr = qmetaobject::QPointer::from(&*cracker);
        let result = qmetaobject::queued_callback(move |(input, output)| {
            if let Some(this) = ptr.as_pinned() {
                this.borrow().found(input, output)
            }
        });
        Self {
            progress_callback: Box::new(progress),
            result_callback: Box::new(result),
        }
    }
}

impl channel::Channel for Channel {
    fn progress(&self, progress: u8) {
        (self.progress_callback)(progress)
    }

    fn result(&self, input: &str, output: &str) {
        (self.result_callback)((String::from(input), String::from(output)))
    }

    fn should_terminate(&self) -> bool {
        false
    }
}
