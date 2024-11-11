use alloc::vec::Vec;
use tree_ds::prelude::{Node, Tree};

use crate::{
    type_info::{GenJsBinding, ValueType},
    utils::ContainerPath,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Container {
    pub path: ContainerPath<'static>,
    pub name: &'static str,
    pub r#type: BindingType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingType {
    Struct(StructType),
    TupleStruct(TupleStructType),
    UnitStruct(UnitStructType),
    Enum(EnumType),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
// encoded into | variant index | (inner)
pub struct EnumType {
    pub variants: Vec<EnumVariant>,
}

impl EnumType {
    pub fn new() -> Self {
        Self::default()
    }

    // index is set based on order of variant registration
    pub fn register_variant(&mut self, name: &'static str) {
        self.variants.push(EnumVariant {
            index: self.variants.len(),
            name,
            inner_type: EnumVariantType::Empty,
        });
    }

    pub fn register_variant_tuple(&mut self, name: &'static str, fields: TupleFields) {
        self.variants.push(EnumVariant {
            index: self.variants.len(),
            name,
            inner_type: EnumVariantType::Tuple(fields.into_inner()),
        });
    }

    pub fn register_unnamed_struct(&mut self, name: &'static str, fields: StructFields) {
        self.variants.push(EnumVariant {
            index: self.variants.len(),
            name,
            inner_type: EnumVariantType::NewType(fields.into_inner()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumVariant {
    pub index: usize,
    pub name: &'static str,
    pub inner_type: EnumVariantType,
}

impl AsRef<EnumVariant> for EnumVariant {
    fn as_ref(&self) -> &EnumVariant {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumVariantType {
    Empty,
    Tuple(Vec<ValueType>),
    // for unnamed structs create struct with custom name ( __EnumName_Struct1)
    NewType(Vec<StructField>),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StructType {
    pub fields: Vec<StructField>,
}

impl StructType {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_field<T: GenJsBinding>(&mut self, name: &'static str) {
        self.fields.push(StructField {
            name,
            v_type: T::get_type(),
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TupleStructType {
    pub fields: Vec<ValueType>,
}

impl TupleStructType {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_field<T: GenJsBinding>(&mut self) {
        self.fields.push(T::get_type())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UnitStructType;

impl UnitStructType {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub name: &'static str,
    pub v_type: ValueType,
}

#[derive(Debug, Default)]
pub struct StructFields(Vec<StructField>);

impl StructFields {
    pub fn register_field<T: GenJsBinding>(&mut self, name: &'static str) {
        self.0.push(StructField {
            name,
            v_type: T::get_type(),
        })
    }

    fn into_inner(self) -> Vec<StructField> {
        self.0
    }
}

#[derive(Default)]
pub struct TupleFields(Vec<ValueType>);

impl TupleFields {
    pub fn register_field<T: GenJsBinding>(&mut self) {
        self.0.push(T::get_type())
    }

    fn into_inner(self) -> Vec<ValueType> {
        self.0
    }
}

pub struct ContainerCollection(Tree<NodeId, NodeType>);

impl ContainerCollection {
    pub fn all_containers(&self) -> impl Iterator<Item = Container> + '_ {
        self.0
            .get_nodes()
            .iter()
            .filter_map(|node| node.get_value().unwrap().get_container().cloned())
    }

    pub fn containers_per_module(&self) -> (Vec<Container>, Vec<Module<'_>>) {
        let root_node = self.0.get_root_node().unwrap().get_node_id();
        container_and_modules_per_mod(&self.0, &root_node)
    }
}

#[derive(Debug)]
pub struct Module<'a>(&'a Tree<NodeId, NodeType>, NodeId, &'static str);

impl<'a> Module<'a> {
    pub fn name(&self) -> &'static str {
        self.2
    }

    pub fn entries(&self) -> (Vec<Container>, Vec<Module<'a>>) {
        container_and_modules_per_mod(self.0, &self.1)
    }
}

fn container_and_modules_per_mod<'a>(
    tree: &'a Tree<NodeId, NodeType>,
    node_id: &NodeId,
) -> (Vec<Container>, Vec<Module<'a>>) {
    let node = tree.get_node_by_id(&node_id).unwrap();

    node.sort_children(|a, b| {
        let a_height = tree.get_node_height(a).unwrap();
        let b_height = tree.get_node_height(b).unwrap();

        a_height.cmp(&b_height).reverse()
    });

    let mut mods = Vec::new();
    let mut containers = Vec::new();

    for (id, child) in node
        .get_children_ids()
        .iter()
        .map(|id| (id, tree.get_node_by_id(id).unwrap()))
    {
        match child.get_value().unwrap() {
            NodeType::Module(name) => mods.push(Module(tree, *id, name)),
            NodeType::Container(container) => containers.push(container.clone()),
        }
    }

    (containers, mods)
}

type NodeId = u128;

enum PathExists {
    Full(NodeId),
    Partly(NodeId, &'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NodeType {
    Container(Container),
    Module(&'static str),
}

impl NodeType {
    fn get_container(&self) -> Option<&Container> {
        match self {
            NodeType::Container(c) => Some(c),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BindingsRegistry(Tree<NodeId, NodeType>);

impl BindingsRegistry {
    pub fn register_struct_binding(
        &mut self,
        name: &'static str,
        path: ContainerPath<'static>,
        value: StructType,
    ) {
        self.insert_container(Container {
            path,
            name,
            r#type: BindingType::Struct(value),
        });
    }

    pub fn register_tuple_struct_binding(
        &mut self,
        name: &'static str,
        path: ContainerPath<'static>,
        value: TupleStructType,
    ) {
        self.insert_container(Container {
            path,
            name,
            r#type: BindingType::TupleStruct(value),
        });
    }

    pub fn register_unit_struct_binding(
        &mut self,
        name: &'static str,
        path: ContainerPath<'static>,
        value: UnitStructType,
    ) {
        self.insert_container(Container {
            path,
            name,
            r#type: BindingType::UnitStruct(value),
        });
    }

    pub fn register_enum_binding(
        &mut self,
        name: &'static str,
        path: ContainerPath<'static>,
        value: EnumType,
    ) {
        self.insert_container(Container {
            path,
            name,
            r#type: BindingType::Enum(value),
        });
    }

    pub fn into_entries(self) -> ContainerCollection {
        ContainerCollection(self.0)
    }

    fn insert_container(&mut self, container: Container) {
        let mut node = self.0.get_root_node().unwrap();
        let mut parts = container.path.parts().skip(1).peekable();
        let path_exists = loop {
            let part = parts.next();
            let is_last = parts.peek().is_none();

            if let Some(part) = part {
                let node_ids = node.get_children_ids();
                let child = node_ids.iter().find(|child| {
                    let node = self.0.get_node_by_id(child).unwrap();
                    matches!(node.get_value(), Some(NodeType::Module(p)) if p == part)
                });

                if let Some(child) = child {
                    if is_last {
                        break PathExists::Full(*child);
                    }

                    node = self.0.get_node_by_id(child).unwrap();
                } else {
                    break PathExists::Partly(node.get_node_id(), part);
                }
            } else {
                break PathExists::Full(node.get_node_id());
            }
        };

        let node_id = match path_exists {
            PathExists::Full(node_id) => node_id,
            PathExists::Partly(node_id, part) => {
                let mut node = self
                    .0
                    .add_node(
                        Node::new_with_auto_id(Some(NodeType::Module(part))),
                        Some(&node_id),
                    )
                    .unwrap();

                for part in parts {
                    node = self
                        .0
                        .add_node(
                            Node::new_with_auto_id(Some(NodeType::Module(part))),
                            Some(&node),
                        )
                        .unwrap();
                }

                node
            }
        };

        self.0
            .add_node(
                Node::new_with_auto_id(Some(NodeType::Container(container))),
                Some(&node_id),
            )
            .unwrap();
    }
}

impl Default for BindingsRegistry {
    fn default() -> Self {
        let mut tree: Tree<NodeId, NodeType> = Tree::new(None);
        tree.add_node(Node::new_with_auto_id(Some(NodeType::Module("::"))), None)
            .unwrap();
        Self(tree)
    }
}

pub trait JsBindings {
    fn create_bindings(registry: &mut BindingsRegistry);
}

#[cfg(test)]
mod test {
    use crate::registry::{
        BindingsRegistry, EnumType, JsBindings, StructFields, StructType, TupleFields,
        TupleStructType,
    };

    #[test]
    fn test_registry_struct() {
        #[allow(unused)]
        struct Test {
            a: u8,
            b: u16,
            c: &'static str,
        }

        impl JsBindings for Test {
            fn create_bindings(registry: &mut BindingsRegistry) {
                let mut ty = StructType::new();

                ty.register_field::<u8>("a".into());
                ty.register_field::<u16>("b".into());
                ty.register_field::<&str>("c".into());

                registry.register_struct_binding("Test", "".into(), ty);
            }
        }

        let mut registry = BindingsRegistry::default();
        Test::create_bindings(&mut registry);
    }

    #[test]
    fn test_registry_tuple_struct() {
        #[allow(dead_code)]
        struct Test(u8, &'static str, &'static [u8]);

        impl JsBindings for Test {
            fn create_bindings(registry: &mut BindingsRegistry) {
                let mut ty = TupleStructType::new();

                ty.register_field::<u8>();
                ty.register_field::<&str>();
                ty.register_field::<&[u8]>();

                registry.register_tuple_struct_binding("Test", "".into(), ty);
            }
        }

        let mut registry = BindingsRegistry::default();
        Test::create_bindings(&mut registry);
    }

    #[test]
    fn test_registry_enum() {
        #[allow(unused)]
        enum Test {
            A,
            B(u8),
            C { a: &'static str, b: u16 },
        }

        impl JsBindings for Test {
            fn create_bindings(registry: &mut BindingsRegistry) {
                let mut ty = EnumType::new();

                ty.register_variant("A".into());

                let mut fields = TupleFields::default();
                fields.register_field::<u8>();
                ty.register_variant_tuple("B".into(), fields);

                let mut fields = StructFields::default();
                fields.register_field::<&str>("a".into());
                fields.register_field::<u16>("b".into());
                ty.register_unnamed_struct("C".into(), fields);

                registry.register_enum_binding("Test", "".into(), ty);
            }
        }

        let mut registry = BindingsRegistry::default();
        Test::create_bindings(&mut registry);
    }
}
