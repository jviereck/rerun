// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs"

#pragma once

#include "../data_cell.hpp"
#include "../datatypes/affix_fuzzer20.hpp"

#include <arrow/type_fwd.h>
#include <cstdint>
#include <utility>

namespace rr {
    namespace components {
        struct AffixFuzzer20 {
            rr::datatypes::AffixFuzzer20 nested_transparent;

            /// Name of the component, used for serialization.
            static const char* NAME;

          public:
            AffixFuzzer20(rr::datatypes::AffixFuzzer20 nested_transparent)
                : nested_transparent(std::move(nested_transparent)) {}

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& to_arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static arrow::Result<std::shared_ptr<arrow::StructBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static arrow::Status fill_arrow_array_builder(
                arrow::StructBuilder* builder, const AffixFuzzer20* elements, size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of AffixFuzzer20 components.
            static arrow::Result<rr::DataCell> to_data_cell(
                const AffixFuzzer20* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rr