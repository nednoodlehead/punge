from punge_ui import Ui_MainWindow
from PyQt5 import QtWidgets as qtw
from PyQt5 import QtCore as qtc


# class used to hold the entire ui... if that makes sense. instance of this class = ui
class PungeMainWindow(qtw.QMainWindow, Ui_MainWindow):

    def __init__(self, parent=None):
        super(PungeMainWindow, self).__init__(parent=parent)
        self.setupUi(self)

        # gives functionality to each of the menu buttons
        self.home_button.clicked.connect(lambda: self.change_page(0))
        self.download_music_menu_button.clicked.connect(lambda: self.change_page(1))
        self.youtube_dl_menu_button.clicked.connect(lambda: self.change_page(2))
        self.settings_menu_button.clicked.connect(lambda: self.change_page(3))
        # automatically change the active page to the first one
        self.change_page(0)

    # shorthand change stackedwidget index. So what page we are on (home, settings, etc...)
    def change_page(self, num):
        self.stackedWidget.setCurrentIndex(num)


if __name__ == "__main__":
    import sys
    app = qtw.QApplication(sys.argv)
    w = PungeMainWindow()
    w.show()
    sys.exit(app.exec_())

