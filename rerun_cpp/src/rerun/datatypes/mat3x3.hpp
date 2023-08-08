// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/mat3x3.fbs"

#pragma once

#include <arrow/type_fwd.h>
#include <cstdint>

namespace rerun {
    namespace datatypes {
        /// A 3x3 column-major Matrix.
        struct Mat3x3 {
            float coeffs[9];

          public:
            Mat3x3() = default;

            Mat3x3(const float (&_coeffs)[9])
                : coeffs{
                      _coeffs[0],
                      _coeffs[1],
                      _coeffs[2],
                      _coeffs[3],
                      _coeffs[4],
                      _coeffs[5],
                      _coeffs[6],
                      _coeffs[7],
                      _coeffs[8]} {}

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& to_arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static arrow::Result<std::shared_ptr<arrow::FixedSizeListBuilder>>
                new_arrow_array_builder(arrow::MemoryPool* memory_pool);

            /// Fills an arrow array builder with an array of this type.
            static arrow::Status fill_arrow_array_builder(
                arrow::FixedSizeListBuilder* builder, const Mat3x3* elements, size_t num_elements
            );
        };
    } // namespace datatypes
} // namespace rerun