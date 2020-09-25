use askama::Template;
use kuchiki::NodeRef;
use serde::Serialize;

use crate::item::Item;
use crate::node_ref_ext::*;

#[derive(Serialize, Clone, Builder, Debug, Default, Template)]
#[template(path = "folder.j2", escape = "none")]
#[builder(setter(into))]
pub struct Folder {
    #[builder(default)]
    title: String,
    #[builder(default = "false")]
    folded: bool,
    #[builder(default)]
    add_date: String,
    #[builder(default)]
    last_modified: String,
    #[builder(default = "false")]
    personal_toolbar_folder: bool,
    #[builder(default = "false")]
    unfiled_bookmarks_folder: bool,
    #[builder(default)]
    children: Vec<Item>,
}

impl Folder {
    pub fn from_node(node: &NodeRef) -> Option<Self> {
        let mut folder = None;

        if node.is_element("DT") {
            let h3 = node.children().find(|n| n.is_element("H3"));

            if let Some(node) = h3 {
                folder = Folder::from_node(&node);
            }
        } else if node.is_element("H3") {
            let mut builder = FolderBuilder::default();

            if node.select_attribute("FOLDED").is_some() {
                builder.folded(true);
            }

            if let Some(attribute) = node.select_attribute("ADD_DATE") {
                builder.add_date(attribute.value);
            }

            if let Some(attribute) = node.select_attribute("LAST_MODIFIED") {
                builder.last_modified(attribute.value);
            }

            if node.select_attribute("PERSONAL_TOOLBAR_FOLDER").is_some() {
                builder.personal_toolbar_folder(true);
            }

            if node.select_attribute("UNFILED_BOOKMARKS_FOLDER").is_some() {
                builder.unfiled_bookmarks_folder(true);
            }

            builder.title(node.text_contents());

            for sibling in node.following_siblings() {
                if sibling.is_element("DL") {
                    let mut children = vec![];

                    for child in sibling.children() {
                        if let Some(item) = Item::from_node(&child) {
                            children.push(item)
                        }
                    }

                    builder.children(children);
                }
            }

            if let Ok(built) = builder.build() {
                folder = Some(built);
            }
        }

        folder
    }
}

impl PartialEq for Folder {
    fn eq(&self, other: &Self) -> bool {
        self.add_date == other.add_date
            && self.title == other.title
            && self.children == other.children
    }
}

#[test]
fn render_folder_html() {
    let rendered = r#"<DT><H3 FOLDED ADD_DATE="date" LAST_MODIFIED="date">name</H3>
<DL><p>
</DL><p>"#;
    let folder = Folder {
        title: String::from("name"),
        folded: true,
        personal_toolbar_folder: false,
        unfiled_bookmarks_folder: false,
        last_modified: String::from("date"),
        add_date: String::from("date"),
        children: vec![],
    };

    assert_eq!(folder.render().unwrap(), rendered);
}

#[test]
fn parse_netscape_empty_folder() {
    use kuchiki::parse_html;
    use kuchiki::traits::TendrilSink;

    let item = r#"
    <DT><H3 FOLDED ADD_DATE="date" LAST_MODIFIED="date">title</H3>
    <DL><p>
    </DL><p>"#;
    let h3 = parse_html().one(item).select_first("H3").unwrap();

    assert_eq!(
        Folder::from_node(&h3.as_node()).unwrap(),
        Folder {
            title: String::from("title"),
            folded: true,
            personal_toolbar_folder: false,
            unfiled_bookmarks_folder: false,
            last_modified: String::from("date"),
            add_date: String::from("date"),
            children: vec![]
        }
    )
}

#[test]
fn parse_netscape_nested_folders() {
    use kuchiki::parse_html;
    use kuchiki::traits::TendrilSink;

    let item = r#"
    <DT><H3>nested0</H3>
    <DL><p>
    <DT><H3>nested1</H3>
    <DL><p>
    <DT><H3>nested2</H3>
    <DL><p>
    <DT><H3>nested3</H3>
    </DL><p>
    </DL><p>
    </DL><p>"#;
    let dt = parse_html().one(item).select_first("DT").unwrap();

    let n3 = Item::Subfolder(
        FolderBuilder::default()
            .title("nested3")
            .children(vec![])
            .build()
            .unwrap(),
    );

    let n2 = Item::Subfolder(
        FolderBuilder::default()
            .title("nested2")
            .children(vec![n3])
            .build()
            .unwrap(),
    );

    let n1 = Item::Subfolder(
        FolderBuilder::default()
            .title("nested1")
            .children(vec![n2])
            .build()
            .unwrap(),
    );

    assert_eq!(
        Folder::from_node(&dt.as_node()).unwrap(),
        FolderBuilder::default()
            .title("nested0")
            .children(vec![n1])
            .build()
            .unwrap()
    )
}

#[test]
fn serialize_json_folder() {
    let json = r#"{"title":"title","folded":false,"add_date":"date","last_modified":"date","personal_toolbar_folder":true,"unfiled_bookmarks_folder":false,"children":[]}"#;
    let folder = Folder {
        title: String::from("title"),
        folded: false,
        personal_toolbar_folder: true,
        unfiled_bookmarks_folder: false,
        add_date: String::from("date"),
        last_modified: String::from("date"),
        children: vec![],
    };

    assert_eq!(serde_json::to_string(&folder).unwrap(), json)
}
