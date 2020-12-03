use qt_widgets::cpp_core::{CastInto, Ptr};
use qt_widgets::qt_core::{qs, QBox};
use qt_widgets::qt_gui::QIntValidator;
use qt_widgets::{
    QApplication, QButtonGroup, QComboBox, QFormLayout, QGridLayout, QGroupBox, QLineEdit,
    QPushButton, QRadioButton, QSpinBox, QTabWidget, QVBoxLayout, QWidget,
};

pub fn run() {
    // unsafe {
    //     let style = qt_widgets::QStyleFactory::create(&qs("Fusion"));
    //     QApplication::set_style_q_style(style);
    // }

    QApplication::init(|_| unsafe {
        let root = QWidget::new_0a();
        let layout = QGridLayout::new_1a(&root);
        layout.set_contents_margins_4a(10, 10, 10, 10);

        let tab = QTabWidget::new_1a(&root);

        let crack = crack_tab(&root);
        let hash = hash_tab(&root);

        tab.add_tab_2a(&crack, &qs("Crack"));
        tab.add_tab_2a(&hash, &qs("Hash"));

        layout.add_widget(&tab);
        root.show();

        QApplication::exec()
    });
}

unsafe fn crack_tab(parent: impl CastInto<Ptr<QWidget>>) -> QBox<QWidget> {
    let root = QWidget::new_1a(parent);
    let layout = QGridLayout::new_1a(&root);
    layout.set_contents_margins_4a(5, 5, 5, 5);

    let details = details_group(&root);
    let algorithm = algorithm_group(&root);
    let salt = salt_group(&root);
    let device = device_group(&root);
    let input = input_group(&root);
    let hash = QPushButton::from_q_string_q_widget(&qs("Crack"), &root);

    layout.add_widget(&details);
    layout.add_widget(&algorithm);
    layout.add_widget(&salt);
    layout.add_widget(&device);
    layout.add_widget(&input);
    layout.add_widget(&hash);

    root
}

unsafe fn hash_tab(parent: impl CastInto<Ptr<QWidget>>) -> QBox<QWidget> {
    let root = QWidget::new_1a(parent);
    let layout = QGridLayout::new_1a(&root);
    layout.set_contents_margins_4a(5, 5, 5, 5);

    let algorithm = algorithm_group(&root);
    let salt = salt_group(&root);
    let input = input_group(&root);
    let hash = QPushButton::from_q_string_q_widget(&qs("Hash"), &root);

    layout.add_widget(&algorithm);
    layout.add_widget(&salt);
    layout.add_widget(&input);
    layout.add_widget(&hash);

    root
}

unsafe fn details_group(parent: impl CastInto<Ptr<QWidget>>) -> QBox<QGroupBox> {
    let root = QGroupBox::from_q_string_q_widget(&qs("Details"), parent);
    let layout = QFormLayout::new_1a(&root);
    layout.set_field_growth_policy(
        qt_widgets::q_form_layout::FieldGrowthPolicy::AllNonFixedFieldsGrow,
    );

    let template = QComboBox::new_1a(&root);
    let prefix = QLineEdit::from_q_widget(&root);
    let length = QSpinBox::new_1a(&root);

    template.add_item_q_string(&qs("Custom"));

    let validator = QIntValidator::new_1a(&prefix);
    prefix.set_validator(&validator);

    length.set_range(1, 25);
    length.set_value(12);

    layout.add_row_q_string_q_widget(&qs("Template"), &template);
    layout.add_row_q_string_q_widget(&qs("Prefix"), &prefix);
    layout.add_row_q_string_q_widget(&qs("Length"), &length);

    root
}

unsafe fn algorithm_group(parent: impl CastInto<Ptr<QWidget>>) -> QBox<QGroupBox> {
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

    root
}

unsafe fn salt_group(parent: impl CastInto<Ptr<QWidget>>) -> QBox<QGroupBox> {
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

    root
}

unsafe fn device_group(parent: impl CastInto<Ptr<QWidget>>) -> QBox<QGroupBox> {
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

    root
}

unsafe fn input_group(parent: impl CastInto<Ptr<QWidget>>) -> QBox<QGroupBox> {
    let root = QGroupBox::from_q_string_q_widget(&qs("Input"), parent);
    let layout = QGridLayout::new_1a(&root);

    let input = qt_widgets::QPlainTextEdit::new();
    layout.add_widget(&input);

    root
}
