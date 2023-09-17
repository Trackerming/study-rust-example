//! # Study Rust example Crate
//! > Study Rust example 是按照`The Rust Programming Language`这本书的章节的示例进行学习的代码部分
//!
//! - 每个章节为一个模块，特殊的章节除外比如test、文档注释都是写在之前的示例上进行展示；
//! - 每个章节的每一小节为一个子模块，模块树对应目录，每个子模块暴露一个公共的方法用于main函数进行调用测试
//
pub mod basic_concept;
pub mod common_collection;
pub mod concurrency;
pub mod enum_pattern_match;
pub mod error_handle;
pub mod generic_traits_lifetime;
pub mod iterator_closure;
pub mod object_oriented;
pub mod ownership;
pub mod package_crate_module;
pub mod smart_pointer;
pub mod struct_related;

pub use self::basic_concept::basic_concept::control_flow;
pub use self::common_collection::hash_map_study;
pub use self::enum_pattern_match::enum_pattern_match::match_control_flow;
pub use self::error_handle::result_recorverable;
pub use self::generic_traits_lifetime::mix_usage;
pub use self::iterator_closure::closure_study;
pub use self::ownership::ownership::reference_borrow;
pub use self::package_crate_module::package_crate_module::package_crate_path;
pub use self::struct_related::retangle;
