#[cxx_qt::bridge]
pub mod qobject {

    unsafe extern "C++" {
        include!(<QtCore/QAbstractListModel>);
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");

        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qlist.h");

        type QList_QString = cxx_qt_lib::QList<QString>;

        include!("cxx-qt-lib/qhash.h");
        /// QHash<i32, QByteArray> from cxx_qt_lib
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;

        include!("cxx-qt-lib/qvariant.h");
        /// QVariant from cxx_qt_lib
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qmodelindex.h");
        /// QModelIndex from cxx_qt_lib
        type QModelIndex = cxx_qt_lib::QModelIndex;

        include!("cxx-qt-lib/qvector.h");
        /// QVector<i32> from cxx_qt_lib
        type QVector_i32 = cxx_qt_lib::QVector<i32>;
    }

    #[qenum(FileListModel)]
    /// Roles for the CustomBaseClass list model
    enum Roles {
        /// The index of the row
        Id,
        /// The value of the row
        Path,
    }

    extern "RustQt" {

        #[qobject]
        #[base = "QAbstractListModel"]
        #[qml_element]
        #[qproperty(QList_QString, files)]
        type FileListModel = super::FileListModelRust;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(*mut FileListModel, files_model)]
        type FileServer = super::FileServerRS;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn search(self: Pin<&mut FileServer>, path: &QString);
    }

    unsafe extern "RustQt" {
        /// Inherit the DataChanged signal from the QAbstractListModel base class
        #[inherit]
        #[qsignal]
        fn data_changed(
            self: Pin<&mut FileListModel>,
            top_left: &QModelIndex,
            bottom_right: &QModelIndex,
            roles: &QVector_i32,
        );
    }

    // ANCHOR: book_inherit_qalm_impl_unsafe
    // Create Rust bindings for C++ functions of the base class (QAbstractItemModel)
    extern "RustQt" {
        /// Inherited beginInsertRows from the base class
        #[inherit]
        unsafe fn begin_insert_rows(
            self: Pin<&mut FileListModel>,
            parent: &QModelIndex,
            first: i32,
            last: i32,
        );
        /// Inherited endInsertRows from the base class
        #[inherit]
        unsafe fn end_insert_rows(self: Pin<&mut FileListModel>);

        /// Inherited beginRemoveRows from the base class
        #[inherit]
        unsafe fn begin_remove_rows(
            self: Pin<&mut FileListModel>,
            parent: &QModelIndex,
            first: i32,
            last: i32,
        );
        /// Inherited endRemoveRows from the base class
        #[inherit]
        unsafe fn end_remove_rows(self: Pin<&mut FileListModel>);

        /// Inherited beginResetModel from the base class
        #[inherit]
        unsafe fn begin_reset_model(self: Pin<&mut FileListModel>);
        /// Inherited endResetModel from the base class
        #[inherit]
        unsafe fn end_reset_model(self: Pin<&mut FileListModel>);

        /// Inherited index from the base class
        #[inherit]
        unsafe fn index(
            self: &FileListModel,
            row: i32,
            column: i32,
            parent: &QModelIndex,
        ) -> QModelIndex;
    }
    // ANCHOR_END: book_inherit_qalm_impl_unsafe

    // QAbstractListModel implementation
    // ANCHOR: book_inherit_data_signature
    unsafe extern "RustQt" {
        #[qinvokable]
        #[cxx_override]
        fn data(self: &FileListModel, index: &QModelIndex, role: i32) -> QVariant;
    }

    unsafe extern "RustQt" {
        /// Return the role names for the QAbstractListModel
        #[qinvokable]
        #[cxx_override]
        fn role_names(self: &FileListModel) -> QHash_i32_QByteArray;

        /// Return the row count for the QAbstractListModel
        #[qinvokable]
        #[cxx_override]
        fn row_count(self: &FileListModel, _parent: &QModelIndex) -> i32;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn refresh_whole_model(self: Pin<&mut FileListModel>);
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QByteArray;
use cxx_qt_lib::QHash;
use cxx_qt_lib::QHashPair_i32_QByteArray;
use cxx_qt_lib::QList;
use cxx_qt_lib::QModelIndex;
use cxx_qt_lib::QString;
use cxx_qt_lib::QUrl;
use cxx_qt_lib::QVariant;
use cxx_qt_lib::QVector;
use std::path::Path;

use self::qobject::FileListModel;

pub struct FileServerRS {
    files: Vec<QString>,

    files_model: *mut FileListModel,
}

impl Default for FileServerRS {
    fn default() -> Self {
        Self {
            files: Default::default(),
            files_model: std::ptr::null_mut(),
        }
    }
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
        let mut files = QList::default();

        self.as_ref()
            .files
            .iter()
            .for_each(|v| files.append(v.clone()));
        unsafe {
            if let Some(inner) = self.as_mut().files_model().as_mut() {
                let mut pinned = Pin::new_unchecked(inner);

                println!("changed to {:?}", files.len());
                pinned.as_mut().set_files(files);
                pinned.as_mut().refresh_whole_model();
            }
        }
    }
}

#[derive(Default)]
pub struct FileListModelRust {
    files: QList<QString>,
}

// QAbstractListModel implementation
//
// ANCHOR: book_inherit_data
impl qobject::FileListModel {
    /// Retrieve the data for a given index and role
    pub fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let role = qobject::Roles { repr: role };
        if let Some(value) = self.files.get(index.row() as isize) {
            return match role {
                qobject::Roles::Id => QVariant::from(&index.row()),
                qobject::Roles::Path => QVariant::from(value),
                _ => QVariant::default(),
            };
        }

        QVariant::default()
    }
}
// ANCHOR_END: book_inherit_data

// ANCHOR_END: book_inherit_can_fetch_more

impl qobject::FileListModel {
    /// Return the role names for the QAbstractListModel
    pub fn role_names(&self) -> QHash<QHashPair_i32_QByteArray> {
        let mut roles = QHash::<QHashPair_i32_QByteArray>::default();
        roles.insert(qobject::Roles::Id.repr, QByteArray::from("id"));
        roles.insert(qobject::Roles::Path.repr, QByteArray::from("path"));
        roles
    }

    /// Return the row count for the QAbstractListModel
    pub fn row_count(&self, _parent: &QModelIndex) -> i32 {
        self.files.len() as i32
    }
}

impl qobject::FileListModel {
    pub fn refresh_whole_model(self: Pin<&mut Self>) {
        unsafe {
            let left = self.index(0, 0, &QModelIndex::default());
            let right = self.index(self.files.len() as i32, 0, &QModelIndex::default());
            let roles = QVector::<i32>::default();

            println!("changed called: {}", self.files.len());
            self.data_changed(&left, &right, &roles);
        }
    }
}
