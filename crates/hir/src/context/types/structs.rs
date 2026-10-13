use std::fmt::Debug;

use crate::{StructField, StructType, SymbolPointer, TupleType, helpers::Visible, term::TermId};
use common::DedupPoolStorage;
use common::{
    dedup_pooled,
    pool::{DedupPool, DedupPoolId},
};
use module_loader::FileId;
dedup_pooled!(pub StructsPool {
    structs: StructType,
    tuples: TupleType,
});

impl StructsPool {
    pub fn insert_struct(
        &self,
        name: SymbolPointer,
        fields: Vec<Visible<(SymbolPointer, TermId)>>,
        owner: FileId,
    ) -> DedupPoolId<StructType> {
        let fields = fields
            .into_iter()
            .map(|field| {
                Visible::new(
                    field.visibility,
                    owner,
                    StructField {
                        name: field.data.0,
                        ty: field.data.1,
                    },
                )
            })
            .collect();

        let s = StructType { name, fields };

        self.structs.insert(s)
    }
}

impl Debug for StructsPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructsPool")
            .field("structs", &self.structs)
            .field("tuples", &self.tuples)
            .finish()
    }
}
