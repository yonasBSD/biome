//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_assists_group;

pub mod organize_imports;

declare_assists_group! {
    pub Correctness {
        name : "correctness" ,
        rules : [
            self :: organize_imports :: OrganizeImports ,
        ]
     }
}
