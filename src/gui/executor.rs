use crate::channel;
use crate::decrypt;
use crate::encrypt;
use crate::error;
use crate::files;
use crate::hash;
use crate::options;

use qt_widgets::cpp_core::Ptr;
use qt_widgets::qt_core::{
    qs, MatchFlag, QBox, QString, QStringList, SignalOfInt, SignalOfQString, SlotNoArgs,
    SlotOfQString,
};
use qt_widgets::{
    q_header_view, q_message_box, QDialog, QHBoxLayout, QLabel, QMessageBox, QProgressBar,
    QPushButton, QTableWidget, QTableWidgetItem, QVBoxLayout, QWidget,
};

pub struct Dialog {
    root: QBox<QDialog>,
    progress: QBox<SignalOfInt>,
    result: QBox<SignalOfQString>,
    done: QBox<SignalOfQString>,
    running: *const bool,
}

struct BoolUnleaker(*const bool);

unsafe impl Send for BoolUnleaker {}

impl BoolUnleaker {
    unsafe fn destroy(&self) {
        // Box::from_raw(self.0 as *mut bool);
    }
}

#[derive(Copy, Clone)]
pub struct Channel {
    progress: Ptr<SignalOfInt>,
    result: Ptr<SignalOfQString>,
    done: Ptr<SignalOfQString>,
    running: *const bool,
}

unsafe impl Send for Channel {}

impl Channel {
    unsafe fn done(&self) {
        if *self.running {
            self.done.emit(&QString::new());
        }
    }

    unsafe fn fail(&self, err: &error::Error) {
        if *self.running {
            self.done.emit(&qs(&err.to_string()));
        }
    }
}

impl channel::Channel for Channel {
    fn progress(&self, progress: u8) {
        unsafe {
            if *self.running {
                self.progress.emit(i32::from(progress));
            }
        }
    }

    fn result(&self, input: &str, output: &str) {
        unsafe {
            if *self.running {
                self.result.emit(&qs(format!("{}:{}", input, output)));
            }
        }
    }

    fn should_terminate(&self) -> bool {
        unsafe { !*self.running }
    }
}

impl Dialog {
    pub unsafe fn new(input: &std::collections::HashSet<String>) -> Self {
        let root = QDialog::new_0a();
        let layout = QVBoxLayout::new_1a(&root);

        let (table, result) = Self::build_table(&root, input);
        let (progress_bar, progress) = Self::build_progress_bar(&root);

        let bottom = QWidget::new_1a(&root);
        let bootom_layout = QHBoxLayout::new_1a(&bottom);

        let cancel = QPushButton::from_q_string_q_widget(&qs("Abort"), &bottom);
        let ok = QPushButton::from_q_string_q_widget(&qs("Ok"), &bottom);

        let done = SignalOfQString::new();
        let running = Box::into_raw(Box::new(true));

        bootom_layout.add_widget(&progress_bar);
        bootom_layout.add_widget(&cancel);
        bootom_layout.add_widget(&ok);

        layout.add_widget(&table);
        layout.add_widget(&bottom);

        let root_ptr = root.as_ptr();
        let progress_bar_ptr = progress_bar.as_ptr();
        let cancel_ptr = cancel.as_ptr();

        let when_done = SlotOfQString::new(&root, move |message| {
            if message.is_empty() {
                progress_bar_ptr.hide();
                cancel_ptr.hide();
            } else {
                QMessageBox::from_icon2_q_string_q_flags_standard_button_q_widget(
                    q_message_box::Icon::Warning,
                    &qs("Error"),
                    message,
                    q_message_box::StandardButton::Ok.into(),
                    root_ptr,
                )
                .exec();
            }
        });
        done.connect(&when_done);

        let abort = SlotNoArgs::new(&root, move || {
            progress_bar_ptr.hide();
            cancel_ptr.hide();
            *running = false;
        });
        root.finished().connect(&abort);
        cancel.clicked().connect(&abort);
        ok.clicked().connect(root.slot_accept());

        result.set_parent(&root);
        progress.set_parent(&root);
        done.set_parent(&root);

        Self {
            root,
            progress,
            result,
            done,
            running,
        }
    }

    pub unsafe fn crack<H: hash::Hash>(
        &self,
        input: std::collections::HashSet<String>,
        files: std::collections::HashSet<std::path::PathBuf>,
        salt: Option<String>,
        length: u8,
        prefix: String,
        device: Option<options::Device>,
    ) {
        let channel = self.as_channel();
        let running = BoolUnleaker(self.running);

        std::thread::spawn(move || {
            if let Err(ref err) = std::thread::spawn(move || {
                let mut hashes = input
                    .into_iter()
                    .map(|h| H::from_str(&h))
                    .collect::<Result<_, _>>()?;

                for file in &files {
                    files::read(&mut hashes, file)?;
                }

                decrypt::execute(
                    &options::Decrypt::<H>::new(hashes, files, salt, length, prefix, None, device)?,
                    channel,
                )?;
                channel.done();
                Ok(())
            })
            .join()
            .map_err(error::on_join)
            .and_then(|res| res)
            {
                channel.fail(err);
            }
            running.destroy();
        });

        self.root.exec();
    }

    pub unsafe fn hash<H: hash::Hash>(
        &self,
        input: std::collections::HashSet<String>,
        salt: Option<String>,
    ) {
        let channel = self.as_channel();
        let running = BoolUnleaker(self.running);

        std::thread::spawn(move || {
            if let Err(ref err) = std::thread::spawn(move || {
                encrypt::execute(&options::Encrypt::<H>::new(input, salt)?, channel);
                channel.done();
                Ok(())
            })
            .join()
            .map_err(error::on_join)
            .and_then(|res| res)
            {
                channel.fail(err);
            }
            running.destroy();
        });
        self.root.exec();
    }

    pub unsafe fn as_channel(&self) -> Channel {
        Channel {
            progress: self.progress.as_ptr(),
            result: self.result.as_ptr(),
            done: self.done.as_ptr(),
            running: self.running,
        }
    }

    unsafe fn build_table(
        parent: &QBox<QDialog>,
        input: &std::collections::HashSet<String>,
    ) -> (QBox<QTableWidget>, QBox<SignalOfQString>) {
        // Allowed because it is checked before casting
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let rows = if input.len() > i32::max_value() as usize {
            i32::max_value()
        } else {
            input.len() as i32
        };

        let table = QTableWidget::new_3a(rows, 2, parent);

        // Allowed because it is limited to i32::MAX_VALUE
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        input
            .iter()
            .enumerate()
            .take_while(|(row, _)| row <= &(i32::max_value() as usize))
            .for_each(|(row, input)| {
                let label = QLabel::new();
                table.set_cell_widget(row as i32, 0, label.into_ptr());
                let label = QLabel::new();
                table.set_cell_widget(row as i32, 1, label.into_ptr());
                let item = QTableWidgetItem::from_q_string(&qs(input));
                table.set_item(row as i32, 0, item.into_ptr());
            });

        let headers = QStringList::from_q_string(&qs("Plain"));
        headers.append_q_string(&qs("Hash"));
        table.set_horizontal_header_labels(&headers);
        table
            .horizontal_header()
            .set_section_resize_mode_1a(q_header_view::ResizeMode::ResizeToContents);
        table.set_minimum_width(600);
        table.set_sorting_enabled(true);
        table.vertical_header().set_visible(false);

        let result = SignalOfQString::new();

        let table_ptr = table.as_ptr();
        let result_received = SlotOfQString::new(&table, move |result| {
            enum Merger<'a> {
                None,
                Half(&'a str),
                Full(&'a str, &'a str),
            }
            if let Merger::Full(first, second) = result.to_std_string().split(':').take(2).fold(
                Merger::None,
                |acc, curr| match acc {
                    Merger::None => Merger::Half(curr),
                    Merger::Half(first) => Merger::Full(first, curr),
                    Merger::Full(_, _) => unreachable!(),
                },
            ) {
                let matches = table_ptr.find_items(&qs(first), MatchFlag::MatchExactly.into());
                if let Some(row) = matches
                    .first()
                    .as_ref()
                    .and_then(|ptr| ptr.as_ref())
                    .map(|item| item.row())
                {
                    let item = QTableWidgetItem::from_q_string(&qs(second));
                    table_ptr.set_item(row, 1, item.into_ptr());
                }
            }
        });

        result.connect(&result_received);

        (table, result)
    }

    unsafe fn build_progress_bar(
        parent: &QBox<QDialog>,
    ) -> (QBox<QProgressBar>, QBox<SignalOfInt>) {
        let progress_bar = QProgressBar::new_1a(parent);
        progress_bar.set_range(0, 100);

        let progress = SignalOfInt::new();

        progress.connect(progress_bar.slot_set_value());

        (progress_bar, progress)
    }
}
