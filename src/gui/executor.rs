use crate::channel;
use crate::encrypt;
use crate::error;
use crate::hash;
use crate::options;

use qt_widgets::cpp_core::Ptr;
use qt_widgets::qt_core::{
    qs, MatchFlag, QBox, QStringList, SignalOfInt, SignalOfQString, SlotOfQString,
};
use qt_widgets::{
    q_header_view, q_message_box, QDialog, QLabel, QMessageBox, QProgressBar, QTableWidget,
    QTableWidgetItem, QVBoxLayout, QWidget,
};

pub struct Dialog {
    root: QBox<QDialog>,
    progress: QBox<SignalOfInt>,
    result: QBox<SignalOfQString>,
}

#[derive(Copy, Clone)]
pub struct Channel {
    progress: Ptr<SignalOfInt>,
    result: Ptr<SignalOfQString>,
}

unsafe impl Send for Channel {}

impl channel::Channel for Channel {
    fn progress(&self, progress: u8) {
        unsafe {
            self.progress.emit(i32::from(progress));
        }
    }

    fn result(&self, input: &str, output: &str) {
        unsafe {
            self.result.emit(&qs(format!("{}:{}", input, output)));
        }
    }

    fn should_terminate(&self) -> bool {
        false
    }
}

impl Dialog {
    pub unsafe fn new(parent: Ptr<QWidget>, input: &std::collections::HashSet<String>) -> Self {
        let root = QDialog::new_1a(parent);
        let layout = QVBoxLayout::new_1a(&root);

        let (table, result) = Self::build_table(&root, input);
        let (progress_bar, progress) = Self::build_progress_bar(&root);

        layout.add_widget(&table);
        layout.add_widget(&progress_bar);

        Self {
            root,
            progress,
            result,
        }
    }

    pub unsafe fn hash<H: hash::Hash>(
        &self,
        input: std::collections::HashSet<String>,
        salt: Option<String>,
    ) {
        self.root.show();

        let root_ptr = self.root.as_ptr();
        let channel = self.as_channel();

        if let Err(err) = std::thread::spawn(move || {
            encrypt::execute(&options::Encrypt::<H>::new(input, salt)?, channel);
            Ok(())
        })
        .join()
        .map_err(error::on_join)
        .and_then(|res| res)
        {
            QMessageBox::from_icon2_q_string_q_flags_standard_button_q_widget(
                q_message_box::Icon::Warning,
                &qs("Cannot hash"),
                &qs(&err.to_string()),
                q_message_box::StandardButton::Ok.into(),
                root_ptr,
            )
            .exec();
            root_ptr.hide();
        }
    }

    pub unsafe fn as_channel(&self) -> Channel {
        Channel {
            progress: self.progress.as_ptr(),
            result: self.result.as_ptr(),
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
                if !matches.is_empty() {
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
