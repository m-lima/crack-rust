use crate::decrypt;
use crate::encrypt;
use crate::hash;
use crate::options;
use crate::options::Device;
use crate::results;

use std::collections::HashSet;

use qt_widgets::cpp_core::{CastInto, CppBox, Ptr};
use qt_widgets::qt_core::{qs, QBox, QSignalBlocker, SlotNoArgs, SlotOfInt, SlotOfQString};
use qt_widgets::qt_gui::{QFont, QIntValidator};
use qt_widgets::{
    QApplication, QButtonGroup, QComboBox, QFormLayout, QGridLayout, QGroupBox, QHBoxLayout,
    QLabel, QLineEdit, QPushButton, QRadioButton, QSpinBox, QTabWidget, QVBoxLayout, QWidget,
};

mod template;

#[derive(Copy, Clone)]
struct Printer;

impl results::Reporter for Printer {
    fn progress(&self, progress: u8) {
        eprintln!("\rProgress: {:02}%", progress);
    }

    fn report(&self, input: &str, output: &str) {
        println!("{}:{}", input, output);
    }
}

unsafe fn load_font() -> CppBox<QFont> {
    use qt_widgets::qt_gui::QFontDatabase;

    //  f013
    //  f0ad
    //  f085
    //  f477
    //  f055
    //  f146
    //  f0fe

    let font_index = QFontDatabase::add_application_font(&qs(":/FontAwesome.otf"));
    let families = QFontDatabase::application_font_families(font_index);
    QFont::from_q_string(families.front())
}

unsafe fn button_with_icon(
    icon: &str,
    text: &str,
    parent: impl CastInto<Ptr<QWidget>>,
    font: &CppBox<QFont>,
) -> QBox<QPushButton> {
    use qt_widgets::qt_core::AlignmentFlag;

    let button = QPushButton::from_q_widget(parent);
    let layout = QHBoxLayout::new_1a(&button);

    let icon = QLabel::from_q_string_q_widget(&qs(icon), &button);
    let text = QLabel::from_q_string_q_widget(&qs(text), &button);

    icon.set_alignment(AlignmentFlag::AlignHCenter | AlignmentFlag::AlignVCenter);
    text.set_alignment(AlignmentFlag::AlignHCenter | AlignmentFlag::AlignVCenter);

    icon.set_font(font);

    layout.set_spacing(3);
    layout.set_contents_margins_4a(0, 0, 0, 3);

    layout.add_stretch_0a();
    layout.add_widget(&icon);
    layout.add_widget(&text);
    layout.add_stretch_0a();

    button
}

pub fn run() {
    QApplication::init(|_| unsafe {
        let font = load_font();

        let root = QWidget::new_0a();
        let layout = QGridLayout::new_1a(&root);
        layout.set_contents_margins_4a(10, 10, 10, 10);

        let tab = QTabWidget::new_1a(&root);

        let crack = crack_tab(&root, &font);
        let hash = hash_tab(&root, &font);

        tab.add_tab_2a(&crack, &qs("Crack"));
        tab.add_tab_2a(&hash, &qs("Hash"));

        layout.add_widget(&tab);
        root.show();

        QApplication::exec()
    });
}

unsafe fn crack_tab(parent: impl CastInto<Ptr<QWidget>>, font: &CppBox<QFont>) -> QBox<QWidget> {
    let root = QWidget::new_1a(parent);
    let layout = QGridLayout::new_1a(&root);
    layout.set_contents_margins_4a(5, 5, 5, 5);

    let (details, prefix_fn, length_fn) = details_group(&root);
    let (algorithm, algorithm_fn) = algorithm_group(&root);
    let (salt, salt_fn) = salt_group(&root);
    let (device, device_fn) = device_group(&root);
    let (input, input_fn) = crack_input_group(&root, font);
    let advanced = {
        let button = button_with_icon("\u{f0ad}", "Advanced", &root, font); //QPushButton::from_q_widget(&root);
        button.set_checkable(true);
        button.set_checked(false);
        button
    };
    let crack = button_with_icon("\u{f085}", "Crack", &root, font);

    let crack_clicked = SlotNoArgs::new(&root, move || {
        println!("Prefix: {}", prefix_fn());
        println!("Length: {}", length_fn());
        println!("Algorithm: {}", algorithm_fn());
        println!("Salt: {}", salt_fn().unwrap_or_default());
        println!("Device: {}", device_fn().unwrap_or(Device::GPU));
        match algorithm_fn() {
            hash::Algorithm::sha256 => decrypt::execute(
                &options::Decrypt::new(
                    input_fn()
                        .into_iter()
                        .filter_map(|h| <hash::sha256::Hash as hash::Hash>::from_str(&h).ok())
                        .collect(),
                    HashSet::new(),
                    salt_fn(),
                    length_fn(),
                    prefix_fn(),
                    None,
                    device_fn(),
                )
                .unwrap(),
                Printer,
            ),
            hash::Algorithm::md5 => decrypt::execute(
                &options::Decrypt::new(
                    input_fn()
                        .into_iter()
                        .filter_map(|h| <hash::md5::Hash as hash::Hash>::from_str(&h).ok())
                        .collect(),
                    HashSet::new(),
                    salt_fn(),
                    length_fn(),
                    prefix_fn(),
                    None,
                    device_fn(),
                )
                .unwrap(),
                Printer,
            ),
        };
    });
    crack.clicked().connect(&crack_clicked);

    algorithm.set_visible(false);
    salt.set_visible(false);
    device.set_visible(false);

    advanced.toggled().connect(algorithm.slot_set_visible());
    advanced.toggled().connect(salt.slot_set_visible());
    advanced.toggled().connect(device.slot_set_visible());

    layout.add_widget(&details);
    layout.add_widget(&algorithm);
    layout.add_widget(&salt);
    layout.add_widget(&device);
    layout.add_widget(&input);
    layout.add_widget(&advanced);
    layout.add_widget(&crack);

    root
}

unsafe fn hash_tab(parent: impl CastInto<Ptr<QWidget>>, font: &CppBox<QFont>) -> QBox<QWidget> {
    let root = QWidget::new_1a(parent);
    let layout = QGridLayout::new_1a(&root);
    layout.set_contents_margins_4a(5, 5, 5, 5);

    let (algorithm, algorithm_fn) = algorithm_group(&root);
    let (salt, salt_fn) = salt_group(&root);
    let (input, input_fn) = hash_input_group(&root);
    let advanced = {
        let button = button_with_icon("\u{f0ad}", "Advanced", &root, font); //QPushButton::from_q_widget(&root);
        button.set_checkable(true);
        button.set_checked(false);
        button
    };
    let hash = button_with_icon("\u{f085}", "Hash", &root, font);

    let hash_clicked = SlotNoArgs::new(&root, move || match algorithm_fn() {
        hash::Algorithm::sha256 => encrypt::execute(
            &options::Encrypt::<hash::sha256::Hash>::new(input_fn(), salt_fn()).unwrap(),
            Printer,
        ),
        hash::Algorithm::md5 => encrypt::execute(
            &options::Encrypt::<hash::md5::Hash>::new(input_fn(), salt_fn()).unwrap(),
            Printer,
        ),
    });
    hash.clicked().connect(&hash_clicked);

    algorithm.set_visible(false);
    salt.set_visible(false);

    advanced.toggled().connect(algorithm.slot_set_visible());
    advanced.toggled().connect(salt.slot_set_visible());

    layout.add_widget(&algorithm);
    layout.add_widget(&salt);
    layout.add_widget(&input);
    layout.add_widget(&advanced);
    layout.add_widget(&hash);

    root
}

unsafe fn details_group(
    parent: &QBox<QWidget>,
) -> (QBox<QGroupBox>, impl Fn() -> String, impl Fn() -> u8) {
    let root = QGroupBox::from_q_string_q_widget(&qs("Details"), parent);
    let layout = QFormLayout::new_1a(&root);
    layout.set_field_growth_policy(
        qt_widgets::q_form_layout::FieldGrowthPolicy::AllNonFixedFieldsGrow,
    );

    let template = QComboBox::new_1a(&root);
    let prefix = QLineEdit::from_q_widget(&root);
    let length = QSpinBox::new_1a(&root);

    template.add_item_q_string(&qs("Custom"));
    template.add_items(&template::to_q_list());

    let validator = QIntValidator::new_1a(&prefix);
    prefix.set_validator(&validator);

    length.set_range(1, 25);
    length.set_value(12);

    layout.add_row_q_string_q_widget(&qs("Template"), &template);
    layout.add_row_q_string_q_widget(&qs("Prefix"), &prefix);
    layout.add_row_q_string_q_widget(&qs("Length"), &length);

    let template_ptr = template.as_ptr();
    let prefix_ptr = prefix.as_ptr();
    let length_ptr = length.as_ptr();

    let template_changed = SlotOfInt::new(&template, move |index| {
        // Allowed because we check the range before casting to usize
        #[allow(clippy::cast_sign_loss)]
        if index > 0 {
            if let Some(template) = template::TEMPLATES.get((index - 1) as usize) {
                let _block = QSignalBlocker::from_q_object(length_ptr);
                prefix_ptr.set_text(&qs(template.prefix()));
                length_ptr.set_value(template.length());
            }
        }
    });

    let prefix_edited = SlotOfQString::new(&prefix, move |string| {
        let _block = QSignalBlocker::from_q_object(length_ptr);
        template_ptr.set_current_index(0);
        length_ptr.set_range(string.length() + 1, 25);
    });

    let length_changed = SlotOfInt::new(&prefix, move |_| {
        template_ptr.set_current_index(0);
    });

    template.current_index_changed().connect(&template_changed);
    prefix.text_edited().connect(&prefix_edited);
    length.value_changed().connect(&length_changed);

    (
        root,
        move || prefix.text().to_std_string(),
        move || length.text().to_std_string().parse().unwrap(),
    )
}

unsafe fn algorithm_group(
    parent: &QBox<QWidget>,
) -> (QBox<QGroupBox>, impl Fn() -> hash::Algorithm) {
    let root = QGroupBox::from_q_string_q_widget(&qs("Algorithm"), parent);
    let layout = QVBoxLayout::new_1a(&root);

    let group = QButtonGroup::new_1a(&root);

    let sha256 = QRadioButton::from_q_string_q_widget(&qs("Sha256"), &root);
    let md5 = QRadioButton::from_q_string_q_widget(&qs("Md5"), &root);

    sha256.set_checked(true);

    group.add_button_1a(&sha256);
    group.add_button_1a(&md5);

    layout.add_widget(&sha256);
    layout.add_widget(&md5);

    (root, move || {
        if sha256.is_checked() {
            hash::Algorithm::sha256
        } else {
            hash::Algorithm::md5
        }
    })
}

unsafe fn salt_group(parent: &QBox<QWidget>) -> (QBox<QGroupBox>, impl Fn() -> Option<String>) {
    let root = QGroupBox::from_q_string_q_widget(&qs("Salt"), parent);
    let layout = QVBoxLayout::new_1a(&root);

    let group = QButtonGroup::new_1a(&root);

    let default = QRadioButton::from_q_string_q_widget(&qs("Default"), &root);
    let custom = QRadioButton::from_q_string_q_widget(&qs("Custom"), &root);
    let input = QLineEdit::from_q_widget(&root);

    default.set_checked(true);
    input.set_enabled(false);

    group.add_button_1a(&default);
    group.add_button_1a(&custom);

    layout.add_widget(&default);
    layout.add_widget(&custom);
    layout.add_widget(&input);

    custom.toggled().connect(input.slot_set_enabled());

    (root, move || {
        if default.is_checked() {
            None
        } else {
            Some(input.text().to_std_string())
        }
    })
}

// Allowed because CPU and GPU are well-known names and expected here
#[allow(clippy::similar_names)]
unsafe fn device_group(parent: &QBox<QWidget>) -> (QBox<QGroupBox>, impl Fn() -> Option<Device>) {
    let root = QGroupBox::from_q_string_q_widget(&qs("Device"), parent);
    let layout = QVBoxLayout::new_1a(&root);

    let group = QButtonGroup::new_1a(&root);

    let auto = QRadioButton::from_q_string_q_widget(&qs("Auto-detect"), &root);
    let gpu = QRadioButton::from_q_string_q_widget(&qs("GPU"), &root);
    let cpu = QRadioButton::from_q_string_q_widget(&qs("CPU"), &root);

    auto.set_checked(true);

    group.add_button_1a(&auto);
    group.add_button_1a(&gpu);
    group.add_button_1a(&cpu);

    layout.add_widget(&auto);
    layout.add_widget(&gpu);
    layout.add_widget(&cpu);

    (root, move || {
        if auto.is_checked() {
            None
        } else if gpu.is_checked() {
            Some(Device::GPU)
        } else {
            Some(Device::CPU)
        }
    })
}

unsafe fn crack_input_group(
    parent: &QBox<QWidget>,
    font: &CppBox<QFont>,
) -> (QBox<QGroupBox>, impl Fn() -> HashSet<String>) {
    let root = QGroupBox::from_q_string_q_widget(&qs("Input"), parent);
    let layout = QGridLayout::new_1a(&root);

    let input = qt_widgets::QListWidget::new_1a(&root);
    // for _ in 0..5 {
    // let item = qt_widgets::QListWidgetItem::from_q_string_q_list_widget(
    //     &qs("a878b6be3c969da1f7abdbcf6720223efc47ab40e47dd506226046c9d728e72d"),
    //     &input,
    // );
    // item.set_data(
    //     0,
    //     &qt_widgets::qt_core::QVariant::from_q_string(&qs("text")),
    // );
    // // input.insert_item_int_q_list_widget_item(i, &item);
    input.insert_item_int_q_string(
        0,
        &qs("a878b6be3c969da1f7abdbcf6720223efc47ab40e47dd506226046c9d728e72d"),
    );
    input.insert_item_int_q_string(
        1,
        &qs("939ecdacf1a29ffe38c6d41fcba5a7329751a13271425583e963cf8f253083c0"),
    );
    input.insert_item_int_q_string(
        2,
        &qs("2b330a0ac7a53b97e1107438f37e567d84c9fb6b419cb067ab2d9d34eabae68d"),
    );
    // }

    let buttons = QWidget::new_1a(&root);
    {
        let button_layout = QHBoxLayout::new_1a(&buttons);
        button_layout.set_margin(0);
        button_layout.set_contents_margins_4a(0, 0, 0, 0);
        button_layout.set_spacing(0);

        let add_hash = {
            let button = QPushButton::from_q_string_q_widget(&qs("\u{f0fe}"), &root);
            button.set_font(font);
            button
        };
        let remove = {
            let button = QPushButton::from_q_string_q_widget(&qs("\u{f146}"), &root);
            button.set_font(font);
            button
        };
        let add_file = {
            let button = QPushButton::from_q_string_q_widget(&qs("\u{f477}"), &root);
            button.set_font(font);
            button
        };

        add_hash.set_tool_tip(&qs("Add hash"));
        remove.set_tool_tip(&qs("Remove selected"));
        add_file.set_tool_tip(&qs("Add file"));

        remove.set_enabled(false);

        button_layout.add_widget(&add_hash);
        button_layout.add_widget(&remove);
        button_layout.add_widget(&add_file);
    }

    layout.add_widget(&input);
    layout.add_widget(&buttons);

    (root, move || {
        HashSet::new()
        // input
        //     .to_plain_text()
        //     .to_std_string()
        //     .split('\n')
        //     .filter_map(|line| {
        //         if line.is_empty() {
        //             None
        //         } else {
        //             Some(String::from(line))
        //         }
        //     })
        //     .collect()
    })
}

unsafe fn hash_input_group(
    parent: &QBox<QWidget>,
) -> (QBox<QGroupBox>, impl Fn() -> HashSet<String>) {
    let root = QGroupBox::from_q_string_q_widget(&qs("Input"), parent);
    let layout = QGridLayout::new_1a(&root);

    let input = qt_widgets::QPlainTextEdit::from_q_widget(&root);
    input.set_tool_tip(&qs("Each line will be hashed separately"));
    input.set_placeholder_text(&qs("Lines to hash"));
    layout.add_widget(&input);

    (root, move || {
        input
            .to_plain_text()
            .to_std_string()
            .split('\n')
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(String::from(line))
                }
            })
            .collect()
    })
}
