#[cxx_qt::bridge]
pub mod qobject {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");

        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qlist.h");

        type QList_QString = cxx_qt_lib::QList<QString>;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QList_QString, files_model)]
        type FileServer = super::FileServerRS;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn search(self: Pin<&mut FileServer>, path: &QString);

        // #[qinvokable]
        // fn file(self: &FileServer, index: usize) -> QString;

        // #[qinvokable]
        // fn len(self: &FileServer) -> usize;
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QList;
use cxx_qt_lib::QString;
use cxx_qt_lib::QUrl;
use std::path::Path;

#[derive(Default)]
pub struct FileServerRS {
    files: Vec<QString>,

    files_model: QList<QString>,
}

impl FileServerRS {
    pub fn search_directory(&mut self, path: &str) -> std::io::Result<()> {
        let url = QUrl::from(path);
        let local = url.to_local_file().map(|v| v.to_string());
        let path = local.as_deref().map(Path::new).unwrap_or(Path::new(path));

        if path.is_dir() {
            let mut files = vec![];
            let dir = path.read_dir()?;

            for entry in dir {
                let entry = entry?;
                let filename = entry.file_name();
                let filename = filename.to_str().unwrap();

                println!("Got a file -> {filename}");
                files.push(filename.into());
            }
            self.files = files;
        }
        Ok(())
    }
}

impl qobject::FileServer {
    pub fn search(mut self: Pin<&mut Self>, path: &QString) {
        let path = path.to_string();

        self.as_mut().rust_mut().search_directory(&path).unwrap();
        let mut model = QList::default();

        self.as_ref()
            .files
            .iter()
            .for_each(|v| model.append(v.clone()));
        self.as_mut().set_files_model(model);
    }

    // pub fn file(&self, index: usize) -> QString {
    //     let ret = self
    //         .files
    //         .get(index)
    //         .cloned()
    //         .unwrap_or(QString::from("None"));
    //     println!("request file index => {index} => {ret:?}");
    //     ret
    // }

    // pub fn len(&self) -> usize {
    //     self.files.len()
    // }
}
