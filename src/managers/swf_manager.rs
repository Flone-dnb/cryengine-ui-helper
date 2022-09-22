// External.
use swf::{FileAttributes, Tag};

pub struct SwfManager {}

impl SwfManager {
    pub fn is_uses_action_script_3(path_to_swf: &str) -> bool {
        // Parse SWF.
        let sfw_data = std::fs::read(path_to_swf).unwrap();
        let stream = swf::decompress_swf(&sfw_data[..]).unwrap();
        let swf = swf::parse_swf(&stream).unwrap();

        // Get info.
        let mut _uses_actionscript_3 = false;
        for tag in swf.tags.iter() {
            match tag {
                Tag::FileAttributes(attributes) => {
                    _uses_actionscript_3 = attributes.contains(FileAttributes::IS_ACTION_SCRIPT_3);
                    break;
                }
                _ => {}
            }
        }

        _uses_actionscript_3
    }
}
