use qmetaobject::QObject;

use crate::channel;
use crate::decrypt;
use crate::hash;
use crate::options;

#[allow(non_snake_case, dead_code)]
#[derive(QObject, Default)]
pub struct Cracker {
    base: qmetaobject::qt_base_class!(trait QObject),
    running: qmetaobject::qt_property!(bool; READ is_running WRITE set_running NOTIFY runningChanged),
    runningChanged: qmetaobject::qt_signal!(running: bool),
    progressed: qmetaobject::qt_signal!(progress: u8),
    found: qmetaobject::qt_signal!(input: String, output: String),
    error: qmetaobject::qt_signal!(message: String),
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
        )
    ),
    running_arc: std::sync::Arc<std::sync::atomic::AtomicBool>,
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
        let prev = self
            .running_arc
            .swap(running, std::sync::atomic::Ordering::Relaxed);
        if prev != running {
            self.runningChanged(running);
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
        input: String,
        files: qmetaobject::QVariantList,
    ) {
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
        &mut self,
        prefix: String,
        length: u8,
        custom_salt: bool,
        salt: String,
        auto_device: bool,
        use_gpu: bool,
        input: String,
        files: qmetaobject::QVariantList,
    ) {
        use qmetaobject::QMetaType;

        let input = H::regex()
            .find_iter(&input)
            .filter_map(|m| <hash::md5::Hash as hash::Hash>::from_str(m.as_str()).ok())
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

        match options::Decrypt::new(input, files, maybe_salt, length, prefix, None, maybe_device) {
            Ok(options) => self.launch(options),
            Err(err) => self.error(err.to_string()),
        }
    }

    fn launch<H: hash::Hash>(&mut self, options: options::Decrypt<H>) {
        let channel = Channel::new(self);
        let running_arc = self.running_arc.clone();

        std::thread::spawn(move || {
            running_arc.swap(true, std::sync::atomic::Ordering::Relaxed);

            // TODO: Need to communicate failure
            // TODO: Need to communicate Summary?
            let _ignored = decrypt::execute(&options, &channel);

            running_arc.swap(false, std::sync::atomic::Ordering::Relaxed);
        });
    }
}

struct Channel {
    progress: Box<dyn Fn(u8) + Send + Sync>,
    result: Box<dyn Fn((String, String)) + Send + Sync>,
    running_arc: std::sync::Arc<std::sync::atomic::AtomicBool>,
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
        Self {
            progress: Box::new(progress),
            result: Box::new(result),
            running_arc: cracker.running_arc.clone(),
        }
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
        self.running_arc.load(std::sync::atomic::Ordering::Relaxed)
    }
}
