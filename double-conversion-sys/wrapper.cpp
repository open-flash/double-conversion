#include <double-conversion/double-conversion.h>
#include "wrapper.hpp"

namespace double_conversion {
    bool DoubleToStringConverter_ToShortest(const DoubleToStringConverter& converter, double value, StringBuilder* result_builder) {
        return converter.ToShortest(value, result_builder);
    }
}
