#include <QApplication>
#include <QLabel>
#include <QPushButton>
#include <QTimer>
#include <QVBoxLayout>
#include <QWidget>
#include <cstdint>
#include <exception>
#include <cstdio>

extern "C" int run_window(const char *app_id, const char *title, void *state,
                          std::uint32_t (*increment)(void *), bool smoke_test) {
  try {
    int argc = 1;
    char executable[] = "omarchy-app";
    char *argv[] = {executable, nullptr};
    QApplication app(argc, argv);
    app.setApplicationName(QString::fromUtf8(app_id));
    app.setDesktopFileName(QString::fromUtf8(app_id));
    QWidget window;
    window.setWindowTitle(QString::fromUtf8(title));
    window.resize(560, 340);
    auto *layout = new QVBoxLayout(&window);
    layout->setContentsMargins(32, 32, 32, 32);
    auto *heading = new QLabel(QString::fromUtf8(title), &window);
    heading->setTextFormat(Qt::PlainText);
    auto font = heading->font();
    font.setPointSize(22);
    heading->setFont(font);
    auto *count = new QLabel("Clicks: 0", &window);
    count->setTextFormat(Qt::PlainText);
    auto *button = new QPushButton("Increment", &window);
    auto *quit = new QPushButton("Close", &window);
    layout->addWidget(heading);
    layout->addWidget(count);
    layout->addStretch();
    layout->addWidget(button);
    layout->addWidget(quit);
    QObject::connect(button, &QPushButton::clicked, &window, [=] {
      count->setText(QString("Clicks: %1").arg(increment(state)));
    });
    QObject::connect(quit, &QPushButton::clicked, &window, &QWidget::close);
    window.show();
    if (smoke_test) {
      QTimer::singleShot(0, &window, [=] {
        button->click();
        QCoreApplication::exit(count->text() == "Clicks: 1" ? 0 : 2);
      });
    }
    return app.exec();
  } catch (const std::exception &error) {
    std::fprintf(stderr, "Window startup failed: %s\n", error.what());
    return 1;
  } catch (...) {
    return 1;
  }
}
