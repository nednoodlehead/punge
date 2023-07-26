from punge_ui import Ui_MainWindow
from PyQt5 import QtWidgets as qtw
from PyQt5 import QtCore as qtc
# import punge_backend

# class used to hold the entire ui... if that makes sense. instance of this class = ui
class PungeMainWindow(qtw.QMainWindow, Ui_MainWindow):

    def __init__(self, parent=None):
        self.count = 1  # used to keep track if the play/pause button should display play or pause
        super(PungeMainWindow, self).__init__(parent=parent)
        self.setupUi(self)

        # gives functionality to each of the menu buttons
        self.home_button.clicked.connect(lambda: self.change_page(0))
        self.download_music_menu_button.clicked.connect(lambda: self.change_page(1))
        self.youtube_dl_menu_button.clicked.connect(lambda: self.change_page(2))
        self.settings_menu_button.clicked.connect(lambda: self.change_page(3))
        # automatically change the active page to the first one
        self.change_page(0)

        # giving functionality to the bottom bar
        # functionality to the play button
        self.play_pause_button.clicked.connect(self.play_pause_button_func)
        # functionality for the shuffle button
        self.shuffle_button.clicked.connect(self.shuffle_button_func)

    # shorthand change stackedwidget index. So what page we are on (home, settings, etc...)
    def change_page(self, num):
        self.stackedWidget.setCurrentIndex(num)

    def play_pause_button_func(self):
        # will probably have to have the little cooldown thing here eventually. Maybe? it could be fast enough to handle
        if "border-image: url(:/newPrefix/img/punge_play_new.png)" != self.play_pause_button.styleSheet():
            self.play_pause_button.setStyleSheet("border-image: url(:/newPrefix/img/punge_play_new.png)")
        else:
            self.play_pause_button.setStyleSheet("border-image: url(:/newPrefix/img/punge_pause_new.png)")

    def shuffle_button_func(self):
        if ":/newPrefix/img/shuffle_off_new.png" in self.shuffle_button.styleSheet():
            self.shuffle_button.setStyleSheet("border-image: url(:/newPrefix/img/shuffle_on_new.png)")
        else:
            self.shuffle_button.setStyleSheet("border-image: url(:/newPrefix/img/shuffle_off_new.png)")


if __name__ == "__main__":
    import sys
    app = qtw.QApplication(sys.argv)
    w = PungeMainWindow()
    w.show()
    sys.exit(app.exec_())

