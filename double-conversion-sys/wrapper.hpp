#include <double-conversion/double-conversion.h>

namespace double_conversion {
    bool DoubleToStringConverter_ToShortest(const DoubleToStringConverter& self, double value, StringBuilder* result_builder);

    char* StringBuilder_Finalize(StringBuilder& self);
}
