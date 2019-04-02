use std::{cell::{Cell, RefCell}, rc::Rc, collections::BTreeMap};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{
    application::{Global, Tree},
    backend::Backend,
    properties::*,
    widgets::{Context, State},
};

/// This system is used to initializes the widgets.
pub struct InitSystem {
    pub backend: Rc<RefCell<dyn Backend>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub debug_flag: Rc<Cell<bool>>,
}

impl InitSystem {
    // init css ids.
    fn init_id(&self, node: Entity, ecm: &mut EntityComponentManager) {
        // Add css id to global id map.
        let id = if let Ok(selector) = ecm.borrow_component::<Selector>(node) {
            if let Some(id) = &selector.0.id {
                Some((node, id.clone()))
            } else {
                None
            }
        } else {
            None
        };

        if let Some((entity, id)) = id {
            if let Ok(global) = ecm.borrow_mut_component::<Global>(0) {
                global.id_map.insert(id, entity);
            }
        }
    }

    // Read all initial data from css
    fn read_init_from_theme(&self, context: &mut Context) {
        context.theme.update_widget_theme(&mut context.widget());

        // let mut margin = Thickness::default();
        // let mut padding = Thickness::default();
        // let (mut width, mut height, mut min_width, mut min_height, mut max_width, mut max_height) =
        //     (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        // // todo: update widget by selector method!!!

        // if let Ok(selector) = ecm.borrow_component::<Selector>(node) {
        //     let pad = theme.uint("padding", selector) as f64;
        //     padding.left = pad;
        //     padding.top = pad;
        //     padding.right = pad;
        //     padding.bottom = pad;

        //     let left = theme.uint("padding-left", selector) as f64;;
        //     let top = theme.uint("padding-top", selector) as f64;
        //     let right = theme.uint("padding-right", selector) as f64;
        //     let bottom = theme.uint("padding-bottom", selector) as f64;

        //     if left > 0.0 {
        //         padding.left = left;
        //     }

        //     if top > 0.0 {
        //         padding.top = top;
        //     }

        //     if right > 0.0 {
        //         padding.right = right;
        //     }

        //     if bottom > 0.0 {
        //         padding.bottom = bottom;
        //     }

        //     let mar = theme.uint("margin", selector) as f64;
        //     margin.left = mar;
        //     margin.top = mar;
        //     margin.right = mar;
        //     margin.bottom = mar;

        //     let left = theme.uint("margin-left", selector) as f64;;
        //     let top = theme.uint("margin-top", selector) as f64;
        //     let right = theme.uint("margin-right", selector) as f64;
        //     let bottom = theme.uint("margin-bottom", selector) as f64;

        //     if left > 0.0 {
        //         margin.left = left;
        //     }

        //     if top > 0.0 {
        //         margin.top = top;
        //     }

        //     if right > 0.0 {
        //         margin.right = right;
        //     }

        //     if bottom > 0.0 {
        //         margin.bottom = bottom;
        //     }

        //     width = theme.uint("width", selector) as f64;
        //     height = theme.uint("height", selector) as f64;
        //     min_width = theme.uint("min-width", selector) as f64;
        //     max_width = theme.uint("max-width", selector) as f64;
        //     min_height = theme.uint("min_height", selector) as f64;
        //     max_height = theme.uint("min-max", selector) as f64;
        // }

        // if let Ok(w_padding) = ecm.borrow_mut_component::<Padding>(node) {
        //     w_padding.set_thickness(padding);
        // }

        // if let Ok(w_margin) = ecm.borrow_mut_component::<Margin>(node) {
        //     w_margin.set_thickness(margin);
        // }

        // if let Ok(w_constraint) = ecm.borrow_mut_component::<Constraint>(node) {
        //     w_constraint.set_size(width, height);
        //     w_constraint.set_min_size(min_width, min_height);
        //     w_constraint.set_max_size(max_width, max_height);
        // }
    }
}

impl System<Tree> for InitSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        let mut backend = self.backend.borrow_mut();
        let state_context = backend.state_context();

        if self.debug_flag.get() {
            println!("\n------ Widget tree ------\n");

            print_tree(tree.root, 0, tree, ecm);

            println!("\n------ Widget tree ------\n");
        }

        // init css ids
        for node in tree.into_iter() {
            self.init_id(node, ecm);

            let mut context = Context::new(
                node,
                ecm,
                tree,
                &state_context.event_queue,
                &state_context.theme,
                None,
            );

            if let Some(state) = self.states.borrow().get(&node) {
                state.init(&mut context);
            }


            self.read_init_from_theme(&mut context);
        }
    }
}

fn print_tree(entity: Entity, depth: usize, tree: &Tree, ecm: &mut EntityComponentManager) {
    let name = Name::get(entity, ecm);
    let selector = Selector::get(entity, ecm);

    println!("{}{} (entity: {}{})", "| ".repeat(depth), name, entity, selector);

    for child in tree.children.get(&entity).unwrap() {
        print_tree(*child, depth + 1, tree, ecm);
    }
}