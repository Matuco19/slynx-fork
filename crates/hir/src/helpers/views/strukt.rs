use common::pool::DedupPoolId;

use crate::{
    EnumType, EnumVariantType, StructField, StructType, SymbolPointer, TupleType,
    helpers::{HirViewer, Visible},
    term::TermId,
};

impl HirViewer<'_, DedupPoolId<StructType>> {
    pub fn name(&self) -> SymbolPointer {
        self.hir.types.get_struct_name(self.data)
    }

    pub fn fields(&self) -> &[Visible<StructField>] {
        self.hir.types.get_struct_fields(self.data)
    }
}

impl HirViewer<'_, DedupPoolId<TupleType>> {
    pub fn fields(&self) -> &[TermId] {
        &self.hir.types[self.data].fields
    }
}

impl HirViewer<'_, DedupPoolId<EnumType>> {
    pub fn name(&self) -> SymbolPointer {
        self.hir.types.get_enum_name(self.data)
    }

    pub fn variants(&self) -> &[EnumVariantType] {
        self.hir.types.get_enum_variants(self.data)
    }

    ///Finds the variant with the given `name` on this enum.
    pub fn find_variant(&self, name: SymbolPointer) -> Option<usize> {
        self.hir.types.find_enum_variant(self.data, name)
    }
}
