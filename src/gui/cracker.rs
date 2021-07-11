use qmetaobject::QObject;

use crate::channel;
use crate::decrypt;
use crate::files;
use crate::hash;
use crate::options;
use crate::results;

#[allow(non_snake_case, dead_code)]
#[derive(QObject, Default)]
pub struct Cracker {
    base: qmetaobject::qt_base_class!(trait QObject),
    running: qmetaobject::qt_property!(bool; READ is_running WRITE set_running NOTIFY runningChanged),
    runningChanged: qmetaobject::qt_signal!(running: bool),
    progressed: qmetaobject::qt_signal!(progress: u8),
    found: qmetaobject::qt_signal!(input: String, output: String),
    error: qmetaobject::qt_signal!(message: String),
    save: qmetaobject::qt_method!(
        fn(&self, input: String, output: String, results: qmetaobject::QVariantList)
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
            input: qmetaobject::QVariantList,
            files: qmetaobject::QVariantList,
        ) -> usize
    ),
    running_arc: std::sync::Arc<std::sync::atomic::AtomicBool>,
    last_regex: Option<&'static regex::Regex>,
}

#[allow(
    clippy::needless_pass_by_value,
    clippy::too_many_arguments,
    clippy::fn_params_excessive_bools
)]
impl Cracker {
    fn is_running(&self) -> bool {
        self.running_arc.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn set_running(&mut self, running: bool) {
        let previous = self
            .running_arc
            .swap(running, std::sync::atomic::Ordering::Relaxed);
        if previous != running {
            self.runningChanged(running);
        }
    }

    fn save(&self, input: String, output: String, results: qmetaobject::QVariantList) {
        use qmetaobject::QMetaType;

        let pairs = {
            let mut pairs = Vec::with_capacity(results.len() >> 1);
            let mut iter = results.into_iter();
            while let Some(hash) = iter.next() {
                let hash = match qmetaobject::QString::from_qvariant(hash.clone()) {
                    Some(hash) => hash,
                    None => continue,
                };
                let plain = match iter
                    .next()
                    .and_then(|plain| qmetaobject::QString::from_qvariant(plain.clone()))
                {
                    Some(plain) => plain,
                    None => continue,
                };
                pairs.push(results::Pair::new(hash.to_string(), plain.to_string()));
            }
            pairs
        };

        if let Some(regex) = self.last_regex {
            if let Err(err) = files::write(
                regex,
                &std::path::PathBuf::from(input),
                Some(std::path::PathBuf::from(output)),
                &pairs,
            ) {
                self.error(err.to_string());
            }
        } else {
            self.error(String::from("No results yet"));
        }
    }

    fn crack(
        &mut self,
        prefix: String,
        length: u8,
        custom_salt: bool,
        salt: String,
        use_sha256: bool,
        auto_device: bool,
        use_gpu: bool,
        input: qmetaobject::QVariantList,
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
        input: qmetaobject::QVariantList,
        files: qmetaobject::QVariantList,
    ) -> usize {
        use qmetaobject::QMetaType;

        let mut input = input
            .into_iter()
            .filter_map(|v| qmetaobject::QString::from_qvariant(v.clone()))
            .filter_map(|s| H::from_str(&s.to_string()).ok())
            .collect();

        let files = files
            .into_iter()
            .filter_map(|v| qmetaobject::QString::from_qvariant(v.clone()))
            .filter_map(|s| {
                let path = std::path::PathBuf::from(&s.to_string());
                files::read(&mut input, &path)
                    .map(|_| path)
                    .map_err(|err| self.error(err.to_string()))
                    .ok()
            })
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
        self.set_running(true);

        let channel = Channel::new(self);
        self.last_regex = Some(H::regex());

        let ptr = qmetaobject::QPointer::from(&*self);
        let set_running = qmetaobject::queued_callback(move |running| {
            if let Some(pin) = ptr.as_pinned() {
                pin.borrow_mut().set_running(running);
            }
        });

        std::thread::spawn(move || {
            // Allow the GUI to have breathing room to render
            if options.device() == options::Device::GPU {
                std::thread::sleep(std::time::Duration::from_millis(500));
            }

            if let Err(err) = decrypt::execute(&options, &channel) {
                channel.error(err.to_string());
            }

            set_running(false);
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
            running: cracker.running_arc.clone(),
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
