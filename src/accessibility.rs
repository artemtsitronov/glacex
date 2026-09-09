use accesskit::{
    ActionHandler, ActionRequest, ActivationHandler, DeactivationHandler, Node, NodeId, Tree,
    TreeId, TreeUpdate,
};
use std::sync::{Arc, Mutex};

pub const ROOT_ID: NodeId = NodeId(0);

#[derive(Default)]
pub struct SharedTree(pub Arc<Mutex<Vec<(NodeId, Node)>>>);

impl SharedTree {
    pub fn build_update(&self, focus: Option<NodeId>) -> TreeUpdate {
        let children = self.0.lock().unwrap();

        let mut root = Node::new(accesskit::Role::Window);
        root.set_label("glacex app");
        root.set_children(children.iter().map(|(id, _)| *id).collect::<Vec<_>>());

        let mut nodes = vec![(ROOT_ID, root)];
        nodes.extend(children.iter().cloned());

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(ROOT_ID)),
            tree_id: TreeId::ROOT,
            focus: focus.unwrap_or(ROOT_ID),
        }
    }
}

pub struct AccessibilityActivationHandler {
    pub tree: SharedTree,
}

impl ActivationHandler for AccessibilityActivationHandler {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate> {
        Some(self.tree.build_update(None))
    }
}

pub struct AccessibilityActionHandler;

impl ActionHandler for AccessibilityActionHandler {
    fn do_action(&mut self, request: ActionRequest) {
        log::warn!("accessibility action requested: {request:?}");
    }
}

pub struct AccessibilityDeactivationHandler;

impl DeactivationHandler for AccessibilityDeactivationHandler {
    fn deactivate_accessibility(&mut self) {}
}
