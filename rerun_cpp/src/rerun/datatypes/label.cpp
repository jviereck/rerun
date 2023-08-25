// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/label.fbs"

#include "label.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace datatypes {
        const std::shared_ptr<arrow::DataType>& Label::arrow_datatype() {
            static const auto datatype = arrow::utf8();
            return datatype;
        }

        Result<std::shared_ptr<arrow::StringBuilder>> Label::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (!memory_pool) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::StringBuilder>(memory_pool));
        }

        Error Label::fill_arrow_array_builder(
            arrow::StringBuilder* builder, const Label* elements, size_t num_elements
        ) {
            if (!builder) {
                return Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
            }
            if (!elements) {
                return Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Cannot serialize null pointer to arrow array."
                );
            }

            ARROW_RETURN_NOT_OK(builder->Reserve(static_cast<int64_t>(num_elements)));
            for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                ARROW_RETURN_NOT_OK(builder->Append(elements[elem_idx].value));
            }

            return Error::ok();
        }
    } // namespace datatypes
} // namespace rerun