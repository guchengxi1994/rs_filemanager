use std::fs;

use serde::{Deserialize, Serialize};

use super::file::File;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileOrFolder {
    File(File),
    Folder(Folder),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folder {
    pub children: Vec<FileOrFolder>,
    pub name: String,
    // 如果是根目录，则没有此id
    pub parent_id: Option<i64>,
    pub folder_id: i64,
}

impl PartialEq for Folder {
    fn eq(&self, other: &Self) -> bool {
        self.folder_id == other.folder_id
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[allow(unused_must_use)]
impl Folder {
    pub fn default() -> Self {
        Folder {
            children: vec![],
            name: String::from("root"),
            parent_id: None,
            folder_id: 0,
        }
    }

    pub fn default_with_save_path(s: String) -> Self {
        let json = fs::read_to_string(s);
        match json {
            Ok(j) => {
                let f = serde_json::from_str::<Folder>(&j);
                match f {
                    Ok(f0) => {
                        return f0;
                    }
                    Err(_) => {
                        return Self::default();
                    }
                }
            }
            Err(_) => {
                return Self::default();
            }
        }
    }

    pub fn get_children(&self, folder_id: i64) -> Vec<FileOrFolder> {
        if self.folder_id == folder_id {
            return self.children.clone();
        } else {
            if self.children.len() == 0 {
                return vec![];
            } else {
                for f in self.folders() {
                    let _res = f.get_children(folder_id);
                    if _res.len() == 0 {
                        continue;
                    } else {
                        return _res;
                    }
                }
            }
        }
        vec![]
    }

    pub fn contains(&self, child: &FileOrFolder) -> bool {
        match child {
            FileOrFolder::File(f) => {
                return self.files().contains(&f);
            }
            FileOrFolder::Folder(f) => {
                return self.folders().contains(&f);
            }
        }
    }

    pub fn add_a_file_to_current_folder(&mut self, folder_id: i64, child: File) {
        if self.folder_id == folder_id {
            self.append(FileOrFolder::File(child));
            return;
        } else {
            if self.children.len() == 0 {
                return;
            } else {
                let mut i = 0;
                for ff in &self.children {
                    // i.add_a_folder_to_current_folder(folder_id, child);
                    match ff {
                        FileOrFolder::File(_) => {
                            i += 1;
                            continue;
                        }
                        FileOrFolder::Folder(fo) => {
                            if fo.folder_id == folder_id {
                                let mut _fo = fo.clone();
                                _fo.append(FileOrFolder::File(child));
                                // self.children[i] = fo;
                                std::mem::replace(&mut self.children[i], FileOrFolder::Folder(_fo));
                                break;
                            } else {
                                i += 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn remove_a_file_from_current_folder(&mut self, folder_id: i64, child: File) {
        if self.folder_id == folder_id {
            self.remove(FileOrFolder::File(child));
            return;
        } else {
            if self.children.len() == 0 {
                return;
            } else {
                let mut i = 0;
                for ff in &self.children {
                    match ff {
                        FileOrFolder::File(_) => {
                            i += 1;
                            continue;
                        }
                        FileOrFolder::Folder(fo) => {
                            if fo.folder_id == folder_id {
                                let mut _fo = fo.clone();
                                _fo.remove(FileOrFolder::File(child));
                                // // self.children[i] = fo;
                                std::mem::replace(&mut self.children[i], FileOrFolder::Folder(_fo));
                                break;
                            } else {
                                i += 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }

    /// 直接删除，不保留子数据
    pub fn remove_a_folder_from_current_folder_directly(&mut self, folder_id: i64, child: Folder) {
        if self.folder_id == folder_id {
            self.remove(FileOrFolder::Folder(child));
            return;
        } else {
            if self.children.len() == 0 {
                return;
            } else {
                let mut i = 0;
                for ff in &self.children {
                    match ff {
                        FileOrFolder::File(_) => {
                            i += 1;
                            continue;
                        }
                        FileOrFolder::Folder(fo) => {
                            if fo.folder_id == folder_id {
                                let mut _fo = fo.clone();
                                _fo.remove(FileOrFolder::Folder(child));
                                // // self.children[i] = fo;
                                std::mem::replace(&mut self.children[i], FileOrFolder::Folder(_fo));
                                break;
                            } else {
                                i += 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn move_item_to(&mut self, to_id: i64, item: FileOrFolder) {
        match item {
            FileOrFolder::File(mut f) => {
                if to_id == f.parent_id {
                    return;
                }
                self.remove_a_file_from_current_folder(f.parent_id, f.clone());
                f.parent_id = to_id;
                self.add_a_file_to_current_folder(to_id, f);
            }
            FileOrFolder::Folder(f) => match f.parent_id {
                Some(p) => {
                    if p == to_id {
                        return;
                    }
                    let _folder = self.pop_folder(f.clone());
                    match _folder {
                        Some(mut _f) => {
                            _f.parent_id = Some(to_id);
                            self.add_a_folder_to_current_folder(to_id, _f);
                        }
                        None => {
                            println!("[rust-connot-find-folder-error]");
                        }
                    }
                }
                None => {}
            },
        }
    }

    /// 删除，保留子数据
    pub fn remove_a_folder_from_current_folder_keep_children(
        &mut self,
        folder_id: i64,
        child: Folder,
    ) {
        if self.folder_id == folder_id {
            self.remove(FileOrFolder::Folder(child.clone()));
            let children = child.children;
            for i in children {
                self.append(i);
            }

            return;
        } else {
            if self.children.len() == 0 {
                return;
            } else {
                let mut i = 0;
                for ff in &self.children {
                    match ff {
                        FileOrFolder::File(_) => {
                            i += 1;
                            continue;
                        }
                        FileOrFolder::Folder(fo) => {
                            if fo.folder_id == folder_id {
                                let mut _fo = fo.clone();
                                _fo.remove(FileOrFolder::Folder(child.clone()));
                                // // self.children[i] = fo;
                                std::mem::replace(&mut self.children[i], FileOrFolder::Folder(_fo));
                                let children = child.children;
                                for i in children {
                                    self.append(i);
                                }

                                break;
                            } else {
                                i += 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn add_a_folder_to_current_folder(&mut self, folder_id: i64, child: Folder) {
        if self.folder_id == folder_id {
            self.append(FileOrFolder::Folder(child));
            return;
        } else {
            if self.children.len() == 0 {
                return;
            } else {
                let mut i = 0;
                for ff in &self.children {
                    // i.add_a_folder_to_current_folder(folder_id, child);
                    match ff {
                        FileOrFolder::File(_) => {
                            i += 1;
                            continue;
                        }
                        FileOrFolder::Folder(fo) => {
                            if fo.folder_id == folder_id {
                                let mut _fo = fo.clone();
                                _fo.append(FileOrFolder::Folder(child));
                                // self.children[i] = fo;
                                std::mem::replace(&mut self.children[i], FileOrFolder::Folder(_fo));
                                break;
                            } else {
                                i += 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }

    fn files(&self) -> Vec<&File> {
        let mut v: Vec<&File> = Vec::new();
        for i in &self.children {
            match i {
                FileOrFolder::File(f) => {
                    v.push(f);
                }
                FileOrFolder::Folder(_) => {}
            }
        }
        return v;
    }

    fn folders(&self) -> Vec<&Folder> {
        let mut v: Vec<&Folder> = Vec::new();
        for i in &self.children {
            match i {
                FileOrFolder::File(_) => {}
                FileOrFolder::Folder(f) => v.push(f),
            }
        }
        return v;
    }

    pub fn get_parent_id(&self, current_id: i64) -> i64 {
        let f = self.get_folder_by_id(current_id);
        match f {
            Some(f0) => match f0.parent_id {
                Some(id) => id,
                None => -1,
            },
            None => {
                return -1;
            }
        }
    }

    pub fn get_parent_id_by_item_id(&self, is_folder: bool, item_id: i64) -> i64 {
        let mut res = -1;

        if is_folder {
            let f = self.folders();
            for i in f {
                if i.folder_id == item_id {
                    res = i.folder_id;
                    break;
                } else {
                    let _r = i.get_parent_id_by_item_id(is_folder, item_id);
                    if _r != -1 {
                        res = _r;
                        break;
                    } else {
                        break;
                    }
                }
            }
        } else {
            let f = self.files();
            for i in f {
                if i.file_id == item_id {
                    res = i.file_id;
                    break;
                }
            }
        }
        res
    }

    fn get_folder_by_id(&self, current_id: i64) -> Option<&Folder> {
        if self.folder_id == current_id {
            return Some(self);
        }

        let _folders = self.folders();

        for i in _folders {
            if i.folder_id == current_id {
                return Some(i);
            } else {
                let _r = i.get_folder_by_id(current_id);
                // return i.get_folder_by_id(current_id);
                match _r {
                    Some(_r0) => {
                        if _r0.folder_id == current_id {
                            return Some(_r0);
                        } else {
                            continue;
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    fn contains_id(&self, id: i64) -> bool {
        for i in self.folders() {
            if i.folder_id == id {
                return true;
            }
        }

        false
    }

    pub fn append(&mut self, child: FileOrFolder) {
        if self.contains(&child) {
            return;
        }
        self.children.push(child)
    }

    pub fn remove(&mut self, child: FileOrFolder) {
        self.children.retain(|x| match x {
            FileOrFolder::File(f) => match &child {
                FileOrFolder::File(c0) => {
                    return f != c0;
                }
                FileOrFolder::Folder(_) => {
                    return true;
                }
            },
            FileOrFolder::Folder(fo) => match &child {
                FileOrFolder::File(_) => {
                    return true;
                }
                FileOrFolder::Folder(c1) => {
                    return fo != c1;
                }
            },
        })
    }

    pub fn pop_folder(&mut self, child: Folder) -> Option<Folder> {
        let mut i: i64 = -1;
        let mut x: Option<Folder> = None;
        for f in &self.children {
            match f {
                FileOrFolder::File(_) => {
                    i += 1;
                }
                FileOrFolder::Folder(f) => {
                    i += 1;
                    if child.folder_id == f.folder_id {
                        x = Some(f.clone());
                        break;
                    }
                }
            }
        }

        if i == -1 {
            return None;
        }

        let _ = self.children.remove(i as usize);

        return x;
    }

    pub fn to_file(&self, s: String) {
        let json = serde_json::to_string(self);
        match json {
            Ok(_j) => {
                println!("{:?}", _j);
                let r = fs::write(s, _j.as_str());
                match r {
                    Ok(_) => {}
                    Err(e) => println!("[rust-write-to-file-err-0] : {:?}", e),
                }
            }
            Err(e) => {
                println!("[rust-write-to-file-err] : {:?}", e)
            }
        }
    }
}
