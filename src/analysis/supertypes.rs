use std::vec::Vec;

use analysis::rust_type::used_rust_type;
use analysis::namespaces;
use env::Env;
use super::general::StatusedTypeId;
use super::imports::Imports;
use library::TypeId;

pub fn analyze(env: &Env, type_id: TypeId, imports: &mut Imports) -> Vec<StatusedTypeId> {
    let mut parents = Vec::new();
    let gobject_id = env.library.find_type(0, "GObject.Object").unwrap();

    for &super_tid in env.class_hierarchy.supertypes(type_id) {
        // skip GObject, it's inherited implicitly
        if super_tid == gobject_id {
            continue;
        }

        let status = env.type_status(&super_tid.full_name(&env.library));

        parents.push(StatusedTypeId {
            type_id: super_tid,
            name: env.library.type_(super_tid).get_name(),
            status,
        });

        if !status.ignored() {
            if let Ok(s) = used_rust_type(env, super_tid) {
                if super_tid.ns_id == namespaces::MAIN {
                    imports.add(&s, None);
                } else {
                    let ns = &env.namespaces[super_tid.ns_id];
                    imports.add(&ns.crate_name, None);
                    imports.add(&ns.ffi_crate_name, None);
                }
            }
        }
    }

    parents
}

pub fn dependencies(env: &Env, type_id: TypeId) -> Vec<TypeId> {
    let mut parents = Vec::new();
    let gobject_id = match env.library.find_type(0, "GObject.Object") {
        Some(gobject_id) => gobject_id,
        None => TypeId::tid_none(),
    };

    for &super_tid in env.class_hierarchy.supertypes(type_id) {
        // skip GObject, it's inherited implicitly
        if super_tid == gobject_id {
            continue;
        }

        let status = env.type_status(&super_tid.full_name(&env.library));

        if status.need_generate() {
            parents.push(super_tid);
        }
    }

    parents
}
