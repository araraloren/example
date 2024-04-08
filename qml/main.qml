import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs
import QtQuick.Controls.Material
import com.demo.rs

ApplicationWindow {
    height: 500
    width: 300
    title: qsTr("Demo Window")
    visible: true

    FileServer {
        id: fileServer
        filesModel: FileListModel { 
            id: fileListModel
            onDataChanged: {
                console.log("1234");
                console.log(fileListModel);
            }
        }
    }

    Component.onCompleted: {
        fileServer.search(".");
    }

    FolderDialog {
        id: folderDialog 
        onAccepted: {
            location.text = folderDialog.selectedFolder
            console.log(location.text)
            fileServer.search(location.text)
        }
    }

    ColumnLayout {
        anchors.fill: parent
        GroupBox {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.verticalStretchFactor: 1
            TextField {
                id: location
                anchors.fill: parent
                onPressed: {
                    folderDialog.open()
                }
            }
        }
        GroupBox {
            title: "Files"
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.verticalStretchFactor: 10
            ListView {
                id: listView
                model: fileServer.filesModel
                spacing: 5
                anchors.fill: parent
                delegate: ItemDelegate  {
                    width: ListView.view.width - 10
                    height: 40

                    Rectangle {
                        anchors.fill: parent
                        color: "lightgrey"
                        Button {
                            // horizontalAlignment: Qt.AlignHCenter
                            text: "test test"
                            anchors.fill: parent
                            onClicked: {
                                console.log("123")
                            }
                        }
                    }    
                }
                ListView.onAdd: {
                    console.log("listView changed ")
                }
            }
        }
    }
}
