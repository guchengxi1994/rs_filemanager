use crate::model::{self, file::File, folder::Folder};

#[test]
fn test_append() {
    {
        let f1 = File {
            path: String::from("aaaaa"),
            parent_id: 0,
            file_id: 2,
        };
        let f2 = File {
            path: String::from("bbbb"),
            parent_id: 0,
            file_id: 3,
        };
        println!("{:?}", f1 == f2);

        let mut root = Folder {
            name: String::from("root"),
            children: vec![],
            parent_id: None,
            folder_id: 0,
        };
        let sub_folder = Folder {
            name: String::from("sub"),
            children: vec![],
            parent_id: Some(0),
            folder_id: 1,
        };
        root.append(model::folder::FileOrFolder::File(f1));
        root.append(model::folder::FileOrFolder::File(f2));
        root.append(model::folder::FileOrFolder::Folder(sub_folder));

        println!("{:?}", root.children.len());
        println!("{:?}", serde_json::to_string(&root));
    }

    println!("=========================================================");

    {
        let f1 = File {
            path: String::from("aaaaa"),
            parent_id: 0,
            file_id: 0,
        };
        let f2 = File {
            path: String::from("bbbb"),
            parent_id: 0,
            file_id: 1,
        };

        let mut root = Folder {
            name: String::from("root"),
            children: vec![],
            parent_id: None,
            folder_id: 0,
        };
        let sub_folder = Folder {
            name: String::from("sub"),
            children: vec![],
            parent_id: Some(0),
            folder_id: 1,
        };
        root.append(model::folder::FileOrFolder::File(f1.clone()));
        root.append(model::folder::FileOrFolder::File(f2.clone()));
        root.append(model::folder::FileOrFolder::Folder(sub_folder.clone()));
        println!(
            "{:?}",
            root.contains(&model::folder::FileOrFolder::File(f1.clone()))
        );
        println!(
            "{:?}",
            root.contains(&model::folder::FileOrFolder::File(f2))
        );
        println!(
            "{:?}",
            root.contains(&model::folder::FileOrFolder::Folder(sub_folder.clone()))
        );

        println!(
            "{:?}",
            sub_folder.contains(&model::folder::FileOrFolder::File(f1))
        );
    }
}
