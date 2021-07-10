use qmetaobject::QObject;

use crate::channel;
use crate::decrypt;
use crate::hash;
use crate::options;

#[derive(QObject, Default)]
pub struct Cracker {
    base: qmetaobject::qt_base_class!(trait QObject),
    progressed: qmetaobject::qt_signal!(progress: u8),
    found: qmetaobject::qt_signal!(input: String, output: String),
    error: qmetaobject::qt_signal!(message: String),
    cancel: qmetaobject::qt_method!(
        fn cancel(&mut self) {
            self.running
                .swap(false, std::sync::atomic::Ordering::Relaxed);
        }
    ),
    crack: qmetaobject::qt_method!(
        fn(
            &mut self,
            prefix: String,
            length: u8,
            customSalt: bool,
            salt: String,
            useSha256: bool,
            autoDevice: bool,
            useGpu: bool,
            input: String,
            files: qmetaobject::QVariantList,
        ) -> usize
    ),
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

#[allow(
    clippy::needless_pass_by_value,
    clippy::too_many_arguments,
    clippy::fn_params_excessive_bools
)]
impl Cracker {
    fn crack(
        &mut self,
        prefix: String,
        length: u8,
        custom_salt: bool,
        salt: String,
        use_sha256: bool,
        auto_device: bool,
        use_gpu: bool,
        input: String,
        files: qmetaobject::QVariantList,
    ) -> usize {
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
            )
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
            )
        }
    }

    fn crack_algorithm<H: hash::Hash>(
        &mut self,
        prefix: String,
        length: u8,
        custom_salt: bool,
        salt: String,
        auto_device: bool,
        use_gpu: bool,
        input: String,
        files: qmetaobject::QVariantList,
    ) -> usize {
        use qmetaobject::QMetaType;

        let input = H::regex()
            .find_iter(&input)
            .filter_map(|m| H::from_str(m.as_str()).ok())
            .collect::<std::collections::HashSet<_>>();

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

        // Allowed because we can't pass `&mut self` to both side of `map_or_else`
        #[allow(clippy::map_unwrap_or)]
        options::Decrypt::new(input, files, maybe_salt, length, prefix, None, maybe_device)
            .map(|options| {
                use options::SharedAccessor;

                let total = options.input().len();
                self.launch(options);
                total
            })
            .unwrap_or_else(|err| {
                self.error(err.to_string());
                0
            })
    }

    fn launch<H: hash::Hash>(&mut self, options: options::Decrypt<H>) {
        let channel = Channel::new(self);
        let running = self.running.clone();

        std::thread::spawn(move || {
            use channel::Channel;

            running.swap(true, std::sync::atomic::Ordering::Relaxed);

            // TODO: No need to send 100% here. Simply handle the non-running case in QML
            match decrypt::execute(&options, &channel) {
                Ok(_) => channel.progress(100),
                Err(err) => channel.error(err.to_string()),
            }

            running.swap(false, std::sync::atomic::Ordering::Relaxed);
        });
    }
}

struct Channel {
    progress: Box<dyn Fn(u8) + Send + Sync>,
    result: Box<dyn Fn((String, String)) + Send + Sync>,
    error: Box<dyn Fn(String) + Send + Sync>,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl Channel {
    fn new(cracker: &Cracker) -> Self {
        let ptr = qmetaobject::QPointer::from(&*cracker);
        let progress = qmetaobject::queued_callback(move |progress| {
            if let Some(pin) = ptr.as_pinned() {
                pin.borrow().progressed(progress);
            }
        });
        let ptr = qmetaobject::QPointer::from(&*cracker);
        let result = qmetaobject::queued_callback(move |(input, output)| {
            if let Some(pin) = ptr.as_pinned() {
                pin.borrow().found(input, output)
            }
        });
        let ptr = qmetaobject::QPointer::from(&*cracker);
        let error = qmetaobject::queued_callback(move |error| {
            if let Some(pin) = ptr.as_pinned() {
                pin.borrow().error(error)
            }
        });
        Self {
            progress: Box::new(progress),
            result: Box::new(result),
            error: Box::new(error),
            running: cracker.running.clone(),
        }
    }

    fn error(&self, error: String) {
        (self.error)(error)
    }
}

impl channel::Channel for Channel {
    fn progress(&self, progress: u8) {
        (self.progress)(progress)
    }

    fn result(&self, input: &str, output: &str) {
        (self.result)((String::from(input), String::from(output)))
    }

    fn should_terminate(&self) -> bool {
        !self.running.load(std::sync::atomic::Ordering::Relaxed)
    }
}
